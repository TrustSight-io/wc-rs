use std::process::Command;
use std::str;

fn run_wc(args: &[&str]) -> (String, String, i32) {
    let output = Command::new("./target/release/wc")
        .args(args)
        .output()
        .expect("Failed to execute wc");

    let stdout = str::from_utf8(&output.stdout).unwrap().to_string();
    let stderr = str::from_utf8(&output.stderr).unwrap().to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    (stdout, stderr, exit_code)
}

fn run_system_wc(args: &[&str]) -> (String, String, i32) {
    let output = Command::new("wc")
        .args(args)
        .output()
        .expect("Failed to execute system wc");

    let stdout = str::from_utf8(&output.stdout).unwrap().to_string();
    let stderr = str::from_utf8(&output.stderr).unwrap().to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    (stdout, stderr, exit_code)
}

fn extract_counts(output: &str) -> (usize, usize, usize) {
    let line = output.lines().next().unwrap();
    let parts: Vec<&str> = line.split_whitespace().collect();

    let lines = parts[0].parse().unwrap();
    let words = parts[1].parse().unwrap();
    let chars = parts[2].parse().unwrap();

    (lines, words, chars)
}

#[test]
fn test_empty_file() {
    let (our_output, _, _) = run_wc(&["tests/data/empty.txt"]);
    let (sys_output, _, _) = run_system_wc(&["tests/data/empty.txt"]);

    let our_counts = extract_counts(&our_output);
    let sys_counts = extract_counts(&sys_output);

    assert_eq!(our_counts, sys_counts);
    assert_eq!(our_counts, (0, 0, 1)); // Empty file has 1 byte (newline)
}

#[test]
fn test_trivial_file() {
    let (our_output, _, _) = run_wc(&["tests/data/trivial.txt"]);
    let (sys_output, _, _) = run_system_wc(&["tests/data/trivial.txt"]);

    let our_counts = extract_counts(&our_output);
    let sys_counts = extract_counts(&sys_output);

    assert_eq!(our_counts, sys_counts);
    assert_eq!(our_counts.1, 1); // Should have 1 word
}

#[test]
fn test_whitespace_file() {
    let (our_output, _, _) = run_wc(&["tests/data/whitespace.txt"]);
    let (sys_output, _, _) = run_system_wc(&["tests/data/whitespace.txt"]);

    let our_counts = extract_counts(&our_output);
    let sys_counts = extract_counts(&sys_output);

    assert_eq!(our_counts, sys_counts);
    assert_eq!(our_counts.1, 0); // Should have 0 words (only whitespace)
}

#[test]
fn test_small_file() {
    let (our_output, _, _) = run_wc(&["tests/data/small.txt"]);
    let (sys_output, _, _) = run_system_wc(&["tests/data/small.txt"]);

    let our_counts = extract_counts(&our_output);
    let sys_counts = extract_counts(&sys_output);

    assert_eq!(our_counts, sys_counts);

    // Verify reasonable counts for our small test file
    assert!(our_counts.0 > 5); // More than 5 lines
    assert!(our_counts.1 > 20); // More than 20 words
    assert!(our_counts.2 > 100); // More than 100 characters
}

#[test]
fn test_large_file() {
    let (our_output, _, _) = run_wc(&["tests/data/large.txt"]);
    let (sys_output, _, _) = run_system_wc(&["tests/data/large.txt"]);

    let our_counts = extract_counts(&our_output);
    let sys_counts = extract_counts(&sys_output);

    assert_eq!(our_counts, sys_counts);

    // Verify this is actually a large file
    assert!(our_counts.0 > 10000); // More than 10k lines
    assert!(our_counts.1 > 100000); // More than 100k words
    assert!(our_counts.2 > 1000000); // More than 1M characters
}

#[test]
fn test_lines_only_flag() {
    let (our_output, _, _) = run_wc(&["-l", "tests/data/small.txt"]);
    let (sys_output, _, _) = run_system_wc(&["-l", "tests/data/small.txt"]);

    // Extract just the number (first field)
    let our_lines: usize = our_output
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let sys_lines: usize = sys_output
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();

    assert_eq!(our_lines, sys_lines);
}

#[test]
fn test_words_only_flag() {
    let (our_output, _, _) = run_wc(&["-w", "tests/data/small.txt"]);
    let (sys_output, _, _) = run_system_wc(&["-w", "tests/data/small.txt"]);

    let our_words: usize = our_output
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let sys_words: usize = sys_output
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();

    assert_eq!(our_words, sys_words);
}

#[test]
fn test_chars_only_flag() {
    let (our_output, _, _) = run_wc(&["-c", "tests/data/small.txt"]);
    let (sys_output, _, _) = run_system_wc(&["-c", "tests/data/small.txt"]);

    let our_chars: usize = our_output
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let sys_chars: usize = sys_output
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();

    assert_eq!(our_chars, sys_chars);
}

#[test]
fn test_multiple_files() {
    let files = [
        "tests/data/small.txt",
        "tests/data/trivial.txt",
        "tests/data/empty.txt",
    ];

    let (our_output, _, _) = run_wc(&files);
    let (sys_output, _, _) = run_system_wc(&files);

    let our_lines: Vec<&str> = our_output.lines().collect();
    let sys_lines: Vec<&str> = sys_output.lines().collect();

    assert_eq!(our_lines.len(), sys_lines.len());

    // Check that each line has the same counts
    for (our_line, sys_line) in our_lines.iter().zip(sys_lines.iter()) {
        let our_parts: Vec<&str> = our_line.split_whitespace().collect();
        let sys_parts: Vec<&str> = sys_line.split_whitespace().collect();

        // Compare the numeric parts (first 3 elements)
        assert_eq!(our_parts[0], sys_parts[0]); // lines
        assert_eq!(our_parts[1], sys_parts[1]); // words
        assert_eq!(our_parts[2], sys_parts[2]); // chars
    }
}

#[test]
fn test_stdin_processing() {
    use std::io::Write;
    use std::process::Stdio;

    let test_input = "Hello world\nThis is a test\n";

    // Test our implementation
    let mut our_child = Command::new("./target/release/wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn wc");

    our_child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(test_input.as_bytes())
        .unwrap();
    let our_output = our_child.wait_with_output().unwrap();
    let our_stdout = str::from_utf8(&our_output.stdout).unwrap();

    // Test system implementation
    let mut sys_child = Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn system wc");

    sys_child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(test_input.as_bytes())
        .unwrap();
    let sys_output = sys_child.wait_with_output().unwrap();
    let sys_stdout = str::from_utf8(&sys_output.stdout).unwrap();

    let our_counts = extract_counts(our_stdout);
    let sys_counts = extract_counts(sys_stdout);

    assert_eq!(our_counts, sys_counts);
}

#[test]
fn test_performance_large_file() {
    use std::time::Instant;

    // Warm up
    let _ = run_wc(&["tests/data/large.txt"]);
    let _ = run_system_wc(&["tests/data/large.txt"]);

    // Benchmark our implementation
    let start = Instant::now();
    let (our_output, _, _) = run_wc(&["tests/data/large.txt"]);
    let our_time = start.elapsed();

    // Benchmark system implementation
    let start = Instant::now();
    let (sys_output, _, _) = run_system_wc(&["tests/data/large.txt"]);
    let sys_time = start.elapsed();

    // Verify correctness
    let our_counts = extract_counts(&our_output);
    let sys_counts = extract_counts(&sys_output);
    assert_eq!(our_counts, sys_counts);

    // Print performance comparison
    println!("Our implementation: {our_time:?}");
    println!("System wc: {sys_time:?}");
    println!(
        "Speedup: {:.2}x",
        sys_time.as_secs_f64() / our_time.as_secs_f64()
    );

    // Our implementation should be faster (or at least not significantly slower)
    // Allow some variance for test environment differences
    assert!(our_time.as_millis() <= sys_time.as_millis() * 3);
}
