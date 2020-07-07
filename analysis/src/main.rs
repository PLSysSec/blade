use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt;
use std::path::Path;

const CRITERION_DIR: &'static str = "../target/criterion";

fn main() {
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
        println!("{}:", group_name);
        for result in results {
            let overhead = (result.mean_estimate / ref_value - 1.0) * 100.0_f64;
            println!("{}  {:.1}%", result, overhead);
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
        write!(f, "{:18} {:.1} us", self.name, self.mean_estimate / 1000.0_f64)
    }
}
