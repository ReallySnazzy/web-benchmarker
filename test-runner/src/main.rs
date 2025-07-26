use std::{collections::BTreeSet, fs::File, io::{BufReader, BufWriter}, path::PathBuf, process::Command, time::Duration};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

type TestNumber = u64;

const LATEST_RUN_FILE: &str = "latest-run.json";

#[derive(Clone, Deserialize, Serialize)]
enum TestType {
    PlainText,
    Json,
    Database,
    Html
}

#[derive(Clone, Deserialize, Serialize)]
struct TestParameters {
    threads: u32,
    concurrent_requests: u32
}

#[derive(Deserialize, Serialize)]
struct TestResult {
    parameter: TestParameters,
    metrics: serde_json::Value
}

#[derive(Deserialize, Serialize)]
struct TestRun {
    case_number: TestNumber,
    test_type: TestType,
    test_info: TestInfo,
    run_timestamp: DateTime<Utc>,
    results: Vec<TestResult>
}

#[derive(Clone, Deserialize, Serialize)]
struct TestInfo {
    notes: String,
    language: String,
    framework: String,
    test_type: TestType,
    url: String,
    revision: u32
}

#[derive(Clone, Deserialize, Serialize)]
struct TestCase {
    dir: PathBuf,
    number: TestNumber,
    info: TestInfo
}

fn default_test_parameters() -> Vec<TestParameters> {
    let concurrency_options = vec![1, 25, 100, 1000];
    let threads_options = vec![1, 2, 5];
    return concurrency_options.iter().flat_map(|c|
        threads_options.iter().map(|t|
            TestParameters {
                threads: *t,
                concurrent_requests: *c
            }
        )
    ).collect();
}

fn load_languages() -> BTreeSet<String> {
    let file = File::open("supported-langs.json").expect("Failed to load supported languages");
    let reader = BufReader::new(file);
    return serde_json::from_reader(reader).expect("Failed to parse supported languages file");
}

fn load_run_details() -> Vec<TestRun> {
    // If previous run details doesn't exist, then start with empty runs
    let Ok(file) = File::open(LATEST_RUN_FILE) else {
        return vec![];
    };
    let reader = BufReader::new(file);
    return serde_json::from_reader(reader).expect("Failed to parse latest runs file");
}

fn read_test_info_from_dir(path: &PathBuf) -> TestInfo {
    let test_info_str = std::fs::read_to_string(path.join("test.toml")).expect(
        &format!(
            "Failed to read test.toml for {}",
            path.display()
        )
    );
    toml::from_str(&test_info_str).expect(
        &format!(
            "Failed to parse test.toml for {}",
            path.display()
        )
    )
}

fn load_test_cases() -> Vec<TestCase> {
    let mut result = vec![];
    for path in std::fs::read_dir("./tests/").expect("Cannot enumerate test directory") {
        let path = path.unwrap().path();
        let name = path.file_name().unwrap().to_str().unwrap().to_owned();
        let test_info = read_test_info_from_dir(&path);
        
        result.push(TestCase {
            number: name.parse::<u64>().unwrap(),
            dir: path,
            info: test_info
        });
    }
    return result;
}

fn filter_deleted_tests(previous_run_details: &mut Vec<TestRun>, current_test_cases: &Vec<TestCase>) {
    let current_test_numbers = current_test_cases.iter().map(|case| case.number).collect::<BTreeSet<_>>();
    previous_run_details.retain(|run| current_test_numbers.contains(&run.case_number));
}

fn first_new_test_case<'l, 'm>(previous_run_details: &'m Vec<TestRun>, current_test_cases: &'l Vec<TestCase>) -> Option<&'l TestCase> {
    let previous_test_cases = previous_run_details.iter().map(|run| run.case_number).collect::<BTreeSet<_>>();
    current_test_cases.iter().find(|case| !previous_test_cases.contains(&case.number))
}

fn oldest_test_case<'l, 'm>(previous_run_details: &'m Vec<TestRun>, current_test_cases: &'l Vec<TestCase>) -> &'l TestCase {
    let oldest_run = previous_run_details.iter().min_by(|run_a, run_b| run_a.run_timestamp.cmp(&run_b.run_timestamp)).expect("Failed to find an oldest run");
    current_test_cases.iter().find(|case| case.number == oldest_run.case_number).expect(
        &format!(
            "Oldest run with case number {} does not have corresponding test case", oldest_run.case_number
        )
    )
}

fn check_unsupported_lang(test_cases: &Vec<TestCase>, supported_langs: &BTreeSet<String>) {
    for case in test_cases.iter() {
        if !supported_langs.contains(&case.info.language) {
            panic!("Unsupported language {} in test case {}", case.info.language, case.number);
        }
    }
}

fn run_test(case: &TestCase) -> TestRun {
    println!("Starting devenv up");
    let devenv_proc = Command::new("devenv")
        .args(["up", "-d"])
        .current_dir(&case.dir)
        .spawn()
        .expect(&format!(
            "Failed to start devenv up for test case {}",
            case.number
        ));
    println!("Started devenv with PID {} in dir {}", devenv_proc.id(), case.dir.display());
    const MAX_BUILD_RUN_WAIT_MINUTES: u64 = 10;
    let mut started = false;
    for _ in 1..MAX_BUILD_RUN_WAIT_MINUTES {
        std::thread::sleep(Duration::from_millis(60*1000));
        let request_result = smol::block_on(async {
            surf::get(&case.info.url).recv_string().await
        });
        if request_result.is_ok() {
            started = true;
            break;
        }
        if let Err(e) = request_result {
            println!("Web server not found: {}", e);
        }
    }
    if !started {
        Command::new("devenv")
            .args(["processes", "stop"])
            .current_dir(&case.dir)
            .spawn()
            .expect(&format!(
                "Failed to kill devenv for test case {}",
                case.number
            ));
        panic!("Failed to start test case {}", case.number);
    }
    println!("Found webserver, running rewrk");
    const STANDARD_TEST_DURATION: &str = "15s";
    let mut results = vec![];
    for test_parameter in default_test_parameters() {
        println!(
            "rewrk -c {} -t {} -d {} -h {} --json", 
            &test_parameter.concurrent_requests.to_string(),
            &test_parameter.threads.to_string(),
            STANDARD_TEST_DURATION,
            &case.info.url
        );
        let output = Command::new("rewrk")
            .args([
                "-c",
                &test_parameter.concurrent_requests.to_string(),
                "-t",
                &test_parameter.threads.to_string(),
                "-d",
                STANDARD_TEST_DURATION,
                "-h",
                &case.info.url,
                "--json"
            ])
            .output()
            .expect("Failed to read stdout from rewrk");
        let stdout = String::from_utf8(output.stdout.into_iter().collect::<Vec<_>>()).unwrap();
        let metrics = match serde_json::from_str(&stdout) {
            Ok(m) => m,
            Err(e) => {
                let mut res = serde_json::Map::new();
                res.insert("error".to_string(), serde_json::Value::String(e.to_string()));
                serde_json::Value::Object(res)
            }
        };
        results.push(TestResult {
            parameter: test_parameter.clone(),
            metrics
        });
    }
    Command::new("devenv")
        .args(["processes", "stop"])
        .current_dir(&case.dir)
        .spawn()
        .expect(&format!(
            "Failed to kill devenv for test case {}",
            case.number
        ));
    return TestRun {
        case_number: case.number,
        test_type: case.info.test_type.clone(),
        test_info: case.info.clone(),
        run_timestamp: Utc::now(),
        results
    };
}

fn save_run_details(run_results: &Vec<TestRun>) {
    let output = File::create(LATEST_RUN_FILE).expect("Failed to create or open latest run json");
    let writer = BufWriter::new(output);
    serde_json::to_writer_pretty(writer, run_results).expect("Failed to save latest run json");
}

fn main() {
    let supported_languages = load_languages();
    let mut previous_run_details = load_run_details();
    let test_cases = load_test_cases();
    filter_deleted_tests(&mut previous_run_details, &test_cases);
    check_unsupported_lang(&test_cases, &supported_languages);
    let run_result = match first_new_test_case(&previous_run_details, &test_cases) {
        Some(new_test_case) => run_test(new_test_case),
        None => run_test(oldest_test_case(&previous_run_details, &test_cases))
    };
    previous_run_details.retain(|run| run.case_number != run_result.case_number);
    previous_run_details.push(run_result);
    save_run_details(&previous_run_details);
}
