use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2
1144#
2233#
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0.4444444444444444\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2
9988#
1122#
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1.0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"6
1122#
2228#
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0.001932367149758454\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"100000
3226#
3597#
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0.6296297942426154\n");
    assert!(output.stderr_str().is_empty());
}
