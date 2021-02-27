use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"100 80
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "20.0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"7 6
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "14.285714285714285714\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"99999 99998
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0.00100001000010000100\n");
    assert!(output.stderr_str().is_empty());
}
