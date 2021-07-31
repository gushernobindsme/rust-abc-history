use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"50 50
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Alloy\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"100 0
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Gold\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"0 100
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Silver\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"100 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Alloy\n");
    assert!(output.stderr_str().is_empty());
}
