use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt;
use std::path::Path;

const CRITERION_DIR: &'static str = "../target/criterion";
const DEF_COUNTS_FILE: &'static str = "def_counts.json";

fn main() {
    let def_counts = get_def_counts();
    for group_path in std::fs::read_dir(Path::new(CRITERION_DIR))
        .unwrap_or_else(|e| panic!("Failed to read dir {:?}: {}. \nHint: perhaps you haven't run `make bench`?", CRITERION_DIR, e))
        .filter_map(|direntry| direntry.ok())
        .filter(|direntry| direntry.file_name() != "report")
        .map(|direntry| direntry.path())
    {
        let group_name = group_path.file_name().unwrap().to_str().unwrap();
        let results = results_from_group_path(&group_path);
        let ref_value = {
            let ref_result = &results[0];
            assert_eq!(ref_result.name, "Ref");
            ref_result.mean_estimate
        };
        let group_def_counts = def_counts.get(group_name);
        println!("{}:", group_name);
        for result in results {
            let overhead = (result.mean_estimate / ref_value - 1.0) * 100.0_f64;
            let def_count = match group_def_counts {
                None => "unknown".into(),
                Some(group_def_counts) => group_def_counts.get(&result.name).map(|u| u.to_string()).unwrap_or("unknown".into()),
            };
            println!("{}  {:.1}%   {}", result, overhead, def_count);
        }
        println!();
    }
}

fn results_from_group_path(p: impl AsRef<Path>) -> Vec<BenchmarkResult> {
    let results = std::fs::read_dir(p.as_ref())
        .unwrap()
        .filter_map(|direntry| direntry.ok())
        .filter(|direntry| direntry.file_name() != "report")
        .map(|direntry| direntry.path())
        .map(|path| BenchmarkResult::from_path(path))
        .map(|result| (result.name, result.mean_estimate))
        .collect();
    our_sort(results)
}

fn our_sort(mut results: HashMap<String, f64>) -> Vec<BenchmarkResult> {
    let mut sorted = vec![];

    let insert_if_present = |name: &str, results: &mut HashMap<String, f64>, sorted: &mut Vec<BenchmarkResult>| {
        if let Entry::Occupied(oentry) = results.entry(name.into()) {
            let (name, mean_estimate) = oentry.remove_entry();
            sorted.push(BenchmarkResult { name, mean_estimate });
        }
    };

    insert_if_present("Ref", &mut results, &mut sorted);
    insert_if_present("Baseline no v1.1", &mut results, &mut sorted);
    insert_if_present("Lfence no v1.1", &mut results, &mut sorted);
    insert_if_present("SLH no v1.1", &mut results, &mut sorted);
    insert_if_present("Baseline with v1.1", &mut results, &mut sorted);
    insert_if_present("Lfence with v1.1", &mut results, &mut sorted);
    insert_if_present("SLH with v1.1", &mut results, &mut sorted);

    // now push all remaining entries in any order
    for (name, mean_estimate) in results.drain() {
        sorted.push(BenchmarkResult { name, mean_estimate });
    }

    sorted
}

#[derive(PartialEq, Clone, Debug)]
struct BenchmarkResult {
    /// Name of the benchmark
    name: String,
    /// Point estimate for the mean
    mean_estimate: f64,
}

impl BenchmarkResult {
    fn from_path(p: impl AsRef<Path>) -> Self {
        let p = p.as_ref();
        let benchmark_name = p.file_name().unwrap().to_str().unwrap().into();
        let filename = p.join("new").join("estimates.json");
        let contents = std::fs::read_to_string(&filename).unwrap_or_else(|e| panic!("Failed to read file at {:?}: {}", filename, e));
        let parsed = json::parse(&contents).unwrap();
        Self {
            name: benchmark_name,
            mean_estimate: parsed["mean"]["point_estimate"].as_f64().expect("Couldn't read value as an f64"),
        }
    }
}

impl fmt::Display for BenchmarkResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:20} {:.1} us", self.name, self.mean_estimate / 1000.0_f64)
    }
}

fn get_def_counts() -> HashMap<String, HashMap<String, usize>> {
    let contents = std::fs::read_to_string(DEF_COUNTS_FILE).unwrap_or_else(|e| panic!("Failed to read file at {:?}: {}", DEF_COUNTS_FILE, e));
    let parsed = json::parse(&contents).unwrap();
    parsed.entries()
        .map(|(key, val)| (key.into(), json_to_hashmap(val)))
        .collect()
}

fn json_to_hashmap(parsed: &json::JsonValue) -> HashMap<String, usize> {
    parsed.entries()
        .map(|(key, val)| (key.into(), val.as_usize().expect("Couldn't read value as usize")))
        .collect()
}
