use std::collections::{BTreeMap, HashMap};
use std::collections::hash_map::Entry;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const CRITERION_DIR: &'static str = "../target/criterion";
const DEF_COUNTS_FILE: &'static str = "def_counts.json";
const LATEX_TABLE_FILE: &'static str = "table.tex";
const SHA256_SCALING_JSON_FILE: &'static str = "sha256_scaling.json";

/// Don't include results for these groups (in the primary results) even if the
/// results are present
const BLACKLISTED_GROUPS: &'static [&'static str] = &[
    "tea encrypt",
    "tea decrypt",
    "sha256 of 128 bytes",
    "sha256 of 256 bytes",
    "sha256 of 512 bytes",
    "sha256 of 1024 bytes",
    "sha256 of 2048 bytes",
    "sha256 of 4096 bytes",
    "sha256 of 16384 bytes",
    "sha256 of 32768 bytes",
    "sha256 of 65536 bytes",
];

fn main() {
    let group_results = std::fs::read_dir(Path::new(CRITERION_DIR))
        .unwrap_or_else(|e| panic!("Failed to read dir {:?}: {}. \nHint: perhaps you haven't run `make bench`?", CRITERION_DIR, e))
        .filter_map(|direntry| direntry.ok())
        .filter(|direntry| direntry.file_name() != "report")
        .map(|direntry| direntry.path())
        .map(|group_path| GroupResult::from_group_path(group_path))
        .map(|group_result| (group_result.name.clone(), group_result))
        .collect();
    let group_results = our_group_sort(group_results);

    primary_results(&group_results);
    sha256_scaling_results(&group_results);
}

fn primary_results<'a>(group_results: impl IntoIterator<Item = &'a GroupResult>) {
    let def_counts = get_def_counts();
    let mut latex = File::create(LATEX_TABLE_FILE).unwrap_or_else(|e| panic!("Failed to create file {:?}: {}", LATEX_TABLE_FILE, e));
    writeln!(latex, "{}", latex_table_preamble()).unwrap();
    for group_result in group_results {
        if BLACKLISTED_GROUPS.iter().any(|&blacklisted| blacklisted == &group_result.name) {
            continue;
        }
        let group_def_counts = def_counts.get(first_word(&group_result.name)); // kinda hacky: we match based on just the first whitespace-delimited word of the group name. works for our current groups
        println!("{}:", &group_result.name);
        println!("{:<17} {:8.1} us  -        -", "Ref:", group_result.ref_value / 1000.0_f64);
        writeln!(latex, "\\midrule\n{:<47} & {:<11} & {:6.1} us & {:8} & {:4} & {:6.1} us & {:8} & {:4} \\\\",
            format!("\\multirow{{5}}{{*}}{{{}}}", latex_name_of_group(&group_result.name)), "Ref",
            group_result.ref_value / 1000.0_f64, "-", "-",
            group_result.ref_value / 1000.0_f64, "-", "-",
        ).unwrap();
        for result in &group_result.results {
            let overhead_no_v1_1 = result.mean_estimate_no_v1_1 / group_result.ref_value - 1.0;
            let overhead_with_v1_1 = result.mean_estimate_with_v1_1 / group_result.ref_value - 1.0;
            let (def_count_no_v1_1, def_count_with_v1_1) = match group_def_counts {
                None => (None, None),
                Some(group_def_counts) => match group_def_counts.get(&result.name) {
                    None => (None, None),
                    Some(def_count) => (Some(def_count.no_v1_1.to_string()), Some(def_count.with_v1_1.to_string())),
                }
            };
            println!("{:10} no v1.1: {:6.1} us  {:6}   {}", result.name, result.mean_estimate_no_v1_1 / 1000.0_f64, percent(overhead_no_v1_1), or_unknown(def_count_no_v1_1.clone()));
            println!("{:10} w/ v1.1: {:6.1} us  {:6}   {}", result.name, result.mean_estimate_with_v1_1 / 1000.0_f64, percent(overhead_with_v1_1), or_unknown(def_count_with_v1_1.clone()));
            writeln!(latex, "{:<47} & {:<11} & {:6.1} us & {:8} & {:4} & {:6.1} us & {:8} & {:4} \\\\",
                "", latex_name_of_result(&result.name), result.mean_estimate_no_v1_1 / 1000.0_f64, latex_percent(overhead_no_v1_1), latex_or_unknown(def_count_no_v1_1),
                result.mean_estimate_with_v1_1 / 1000.0_f64, latex_percent(overhead_with_v1_1), latex_or_unknown(def_count_with_v1_1),
            ).unwrap();
        }
        println!();
    }

    writeln!(latex, "{}", latex_table_postamble()).unwrap();
    println!("Full LaTeX table printed to {}", LATEX_TABLE_FILE);
}

fn latex_name_of_group(group_name: &str) -> String {
    if let Some(postfix) = group_name.strip_prefix("sha256 of ") {
        format!("SHA-256 (CT-Wasm), {}", postfix)
    } else if let Some(postfix) = group_name.strip_prefix("chacha20 of ") {
        format!("ChaCha20 (HACL*), {}", postfix)
    } else if let Some(postfix) = group_name.strip_prefix("poly1305 of ") {
        format!("Poly1305 (HACL*), {}", postfix)
    } else {
        match group_name {
            "salsa20" => "Salsa20 (CT-Wasm), 64 bytes".into(),
            "curve25519_51" => "ECDH Curve25519 (HACL*)".into(),
            _ => group_name.into(),
        }
    }
}

fn latex_name_of_result(result_name: &str) -> &str {
    match result_name {
        "Lfence" => "\\tool-F",
        "SLH" => "\\tool-S",
        _ => result_name,
    }
}

fn percent(val: f64) -> String {
    format!("{:.1}%", val * 100.0_f64)
}

fn latex_percent(val: f64) -> String {
    format!("{:.1}\\%", val * 100.0_f64)
}

fn or_unknown(val: Option<String>) -> String {
    val.unwrap_or("unknown".into())
}

fn latex_or_unknown(val: Option<String>) -> String {
    val.unwrap_or("\\X".into())
}

/// In: map of name to GroupResult (which also includes the name)
/// Out: sorted GroupResults
fn our_group_sort(mut results: HashMap<String, GroupResult>) -> Vec<GroupResult> {
    let mut sorted = vec![];

    let insert_if_present = |name: &str, results: &mut HashMap<String, GroupResult>, sorted: &mut Vec<GroupResult>| {
        if let Entry::Occupied(oentry) = results.entry(name.into()) {
            sorted.push(oentry.remove());
        }
    };

    insert_if_present("salsa20", &mut results, &mut sorted);
    insert_if_present("sha256 of 64 bytes", &mut results, &mut sorted);
    insert_if_present("sha256 of 128 bytes", &mut results, &mut sorted);
    insert_if_present("sha256 of 256 bytes", &mut results, &mut sorted);
    insert_if_present("sha256 of 512 bytes", &mut results, &mut sorted);
    insert_if_present("sha256 of 1024 bytes", &mut results, &mut sorted);
    insert_if_present("sha256 of 2048 bytes", &mut results, &mut sorted);
    insert_if_present("sha256 of 4096 bytes", &mut results, &mut sorted);
    insert_if_present("sha256 of 8192 bytes", &mut results, &mut sorted);
    insert_if_present("sha256 of 16384 bytes", &mut results, &mut sorted);
    insert_if_present("sha256 of 32768 bytes", &mut results, &mut sorted);
    insert_if_present("sha256 of 65536 bytes", &mut results, &mut sorted);
    insert_if_present("chacha20 of 8192 bytes", &mut results, &mut sorted);
    insert_if_present("poly1305 of 1024 bytes", &mut results, &mut sorted);
    insert_if_present("poly1305 of 8192 bytes", &mut results, &mut sorted);
    insert_if_present("curve25519_51", &mut results, &mut sorted);

    // now push all remaining entries in any order
    for (_, fbr) in results.drain() {
        sorted.push(fbr);
    }

    sorted
}

fn first_word(string: &str) -> &str {
    string.split_whitespace().next().unwrap()
}

fn sha256_scaling_results<'a>(group_results: impl IntoIterator<Item = &'a GroupResult>) {
    let sha256_groups: BTreeMap<usize, &GroupResult> = group_results
        .into_iter()
        .filter_map(|gres| {
            gres.name.strip_prefix("sha256 of ")
                .map(|size_str| (size_str.strip_suffix(" bytes").unwrap().parse::<usize>().unwrap(), gres))
        })
        .collect();
    assert!(!sha256_groups.is_empty());
    let first_group: &GroupResult = sha256_groups.values().next().unwrap();
    let blade_types = first_group.results.iter().map(|fbr| &fbr.name);
    let mut json_data = json::object::Object::new();

    let mut sizes = Vec::with_capacity(sha256_groups.len());
    let mut runtimes_ns = Vec::with_capacity(sha256_groups.len());
    let mut runtimes_ns_per_byte = Vec::with_capacity(sha256_groups.len());
    for (size, gres) in &sha256_groups {
        sizes.push(*size);
        runtimes_ns.push(gres.ref_value);
        runtimes_ns_per_byte.push(gres.ref_value / (*size as f64));
    }
    json_data.insert("ref", json::object! {
        sizes: sizes,
        runtimes_ns: runtimes_ns,
        runtimes_ns_per_byte: runtimes_ns_per_byte,
    });

    for blade_type in blade_types {
        let mut sizes = Vec::with_capacity(sha256_groups.len());
        let mut runtimes_ns_no_v1_1 = Vec::with_capacity(sha256_groups.len());
        let mut runtimes_ns_with_v1_1 = Vec::with_capacity(sha256_groups.len());
        let mut runtimes_ns_per_byte_no_v1_1 = Vec::with_capacity(sha256_groups.len());
        let mut runtimes_ns_per_byte_with_v1_1 = Vec::with_capacity(sha256_groups.len());
        for (size, gres) in &sha256_groups {
            let fbr = gres.results.iter().find(|fbr| &fbr.name == blade_type).unwrap();
            sizes.push(*size);
            runtimes_ns_no_v1_1.push(fbr.mean_estimate_no_v1_1);
            runtimes_ns_with_v1_1.push(fbr.mean_estimate_with_v1_1);
            runtimes_ns_per_byte_no_v1_1.push(fbr.mean_estimate_no_v1_1 / (*size as f64));
            runtimes_ns_per_byte_with_v1_1.push(fbr.mean_estimate_with_v1_1 / (*size as f64));
        }
        json_data.insert(&format!("{} without v1.1", blade_type), json::object! {
            sizes: sizes.clone(),
            runtimes_ns: runtimes_ns_no_v1_1,
            runtimes_ns_per_byte: runtimes_ns_per_byte_no_v1_1,
        });
        json_data.insert(&format!("{} with v1.1", blade_type), json::object! {
            sizes: sizes,
            runtimes_ns: runtimes_ns_with_v1_1,
            runtimes_ns_per_byte: runtimes_ns_per_byte_with_v1_1,
        });
    }

    let mut jsonfile = File::create(SHA256_SCALING_JSON_FILE).unwrap_or_else(|e| panic!("Failed to create file {:?}: {}", SHA256_SCALING_JSON_FILE, e));
    writeln!(jsonfile, "{}", json::stringify_pretty(json_data, 4)).unwrap();
    println!("JSON for scaling results written to {}", SHA256_SCALING_JSON_FILE);
}

/// The results for an entire benchmark group, eg "sha256 of 1024 bytes"
struct GroupResult {
    name: String,
    ref_value: f64,
    results: Vec<FullBenchmarkResult>,
}

/// The results for a benchmark within a group, eg "Baseline-F"
#[derive(PartialEq, Clone, Debug)]
struct FullBenchmarkResult {
    /// Name of the benchmark (with the v1.1 part stripped)
    name: String,
    /// Point estimate for the mean without v1.1 protections
    mean_estimate_no_v1_1: f64,
    /// Point estimate for the mean with v1.1 protections
    mean_estimate_with_v1_1: f64,
}

impl GroupResult {
    fn from_group_path(p: impl AsRef<Path>) -> Self {
        let p = p.as_ref();
        let group_name = p.file_name().unwrap().to_str().unwrap();
        let mut results: HashMap<String, f64> = std::fs::read_dir(p)
            .unwrap()
            .filter_map(|direntry| direntry.ok())
            .filter(|direntry| direntry.file_name() != "report")
            .map(|direntry| direntry.path())
            .map(|path| BenchmarkResult::from_path(path))
            .map(|result| (result.name, result.mean_estimate))
            .collect();
        let ref_value = results.remove("Ref".into()).unwrap_or_else(|| panic!("Group {:?} is missing a Ref result", group_name));
        let results = pair_results(results);
        Self {
            name: group_name.into(),
            ref_value,
            results: our_results_sort(results),
        }
    }
}

/// Match each no-v1.1 result with the corresponding with-v1.1 result
fn pair_results(mut results: HashMap<String, f64>) -> HashMap<String, FullBenchmarkResult> {
    let mut paired = HashMap::new();

    while let Some((name, mean_estimate)) = pop_from_hashmap(&mut results) {
        if name.ends_with(" no v1.1") {
            let short_name: String = name.trim_end_matches(" no v1.1").into();
            let name_with_v1_1 = {
                let mut name = short_name.clone();
                name.push_str(" with v1.1");
                name
            };
            let mean_with_v1_1 = results.remove(&name_with_v1_1).unwrap_or_else(|| panic!("{} has a no-v1.1 result but no with-v1.1 result", short_name));
            paired.insert(
                short_name.clone(),
                FullBenchmarkResult {
                    name: short_name,
                    mean_estimate_no_v1_1: mean_estimate,
                    mean_estimate_with_v1_1: mean_with_v1_1,
                },
            );
        } else if name.ends_with(" with v1.1") {
            let short_name: String = name.trim_end_matches(" with v1.1").into();
            let name_no_v1_1 = {
                let mut name = short_name.clone();
                name.push_str(" no v1.1");
                name
            };
            let mean_no_v1_1 = results.remove(&name_no_v1_1).unwrap_or_else(|| panic!("{} has a with-v1.1 result but no no-v1.1 result", short_name));
            paired.insert(
                short_name.clone(),
                FullBenchmarkResult {
                    name: short_name,
                    mean_estimate_no_v1_1: mean_no_v1_1,
                    mean_estimate_with_v1_1: mean_estimate,
                },
            );
        } else {
            panic!("Expected name {:?} to end with either \"with v1.1\" or \"no v1.1\"", name);
        }
    }

    paired
}

fn pop_from_hashmap(map: &mut HashMap<String, f64>) -> Option<(String, f64)> {
    let key = map.keys().next()?.clone();
    map.remove_entry(&key)
}

/// In: map of name to FullBenchmarkResult (which also includes the name)
/// Out: sorted FullBenchmarkResults
fn our_results_sort(mut results: HashMap<String, FullBenchmarkResult>) -> Vec<FullBenchmarkResult> {
    let mut sorted = vec![];

    let insert_if_present = |name: &str, results: &mut HashMap<String, FullBenchmarkResult>, sorted: &mut Vec<FullBenchmarkResult>| {
        if let Entry::Occupied(oentry) = results.entry(name.into()) {
            sorted.push(oentry.remove());
        }
    };

    insert_if_present("Ref", &mut results, &mut sorted);
    insert_if_present("Baseline-F", &mut results, &mut sorted);
    insert_if_present("Lfence", &mut results, &mut sorted);
    insert_if_present("Baseline-S", &mut results, &mut sorted);
    insert_if_present("SLH", &mut results, &mut sorted);

    // now push all remaining entries in any order
    for (_, fbr) in results.drain() {
        sorted.push(fbr);
    }

    sorted
}

struct BenchmarkResult {
    /// Name of the benchmark. This will include "no v1.1" or "with v1.1" if appropriate
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

struct DefCount {
    no_v1_1: usize,
    with_v1_1: usize,
}

impl DefCount {
    fn from_json(parsed: &json::JsonValue) -> Self {
        Self {
            no_v1_1: parsed["no_v1_1"].as_usize().expect("Couldn't read value as usize"),
            with_v1_1: parsed["with_v1_1"].as_usize().expect("Couldn't read value as usize"),
        }
    }
}

fn get_def_counts() -> HashMap<String, HashMap<String, DefCount>> {
    let contents = std::fs::read_to_string(DEF_COUNTS_FILE).unwrap_or_else(|e| panic!("Failed to read file at {:?}: {}", DEF_COUNTS_FILE, e));
    let parsed = json::parse(&contents).unwrap();
    parsed.entries()
        .map(|(key, val)| (key.into(), json_to_hashmap(val)))
        .collect()
}

fn json_to_hashmap(parsed: &json::JsonValue) -> HashMap<String, DefCount> {
    parsed.entries()
        .map(|(key, val)| (key.into(), DefCount::from_json(val)))
        .collect()
}

fn latex_table_preamble() -> &'static str {
    r#"
%%% AUTOGENERATED FILE (by `make report` in PLSysSec/blade-benchmarks) - do not edit manually %%%

\centering
\footnotesize
\begin{tabular}{llcccccc}

\toprule
& & \multicolumn{3}{c}{Without v1.1 protections} & \multicolumn{3}{c}{With v1.1 protections} \\
\textbf{Benchmark} & \textbf{Defense} & \textbf{Time} & \textbf{Overhead} & \textbf{Defs}
                                      & \textbf{Time} & \textbf{Overhead} & \textbf{Defs}
\\
"#
}

fn latex_table_postamble() -> &'static str {
    r#"
\bottomrule
\end{tabular}"#
}
