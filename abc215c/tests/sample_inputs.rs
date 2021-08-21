use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"aab 2
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "aba\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"baba 4
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "baab\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"ydxwacbz 40320
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "zyxwdcba\n");
    assert!(output.stderr_str().is_empty());
}
