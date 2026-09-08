use std::process::Command;

fn cli(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_bytetally"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn reports_sample_totals() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/traffic.txt");
    let output = cli(&["analyze", path]);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "IP\tBYTES\n2001:db8::1\t4096\n192.168.0.10\t3072\n192.168.0.20\t512\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn rejects_bad_arguments_and_missing_files() {
    for args in [
        vec!["unknown"],
        vec!["analyze"],
        vec!["analyze", "a", "b"],
        vec![
            "analyze",
            "nonexistent-directory-for-bytetally-test/traffic.txt",
        ],
    ] {
        let output = cli(&args);
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8(output.stderr).unwrap().contains("Erro:"));
    }
}

#[test]
fn help_and_version_succeed() {
    for args in [vec![], vec!["--help"], vec!["--version"]] {
        let output = cli(&args);
        assert!(output.status.success());
        assert!(!output.stdout.is_empty());
        assert!(output.stderr.is_empty());
    }
}

#[test]
fn malformed_file_has_no_partial_report() {
    // This source file is deliberately not a traffic log.
    let output = cli(&[
        "analyze",
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/cli.rs"),
    ]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("linha 1:")
    );
}
