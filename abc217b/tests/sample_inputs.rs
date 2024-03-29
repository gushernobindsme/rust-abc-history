use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"ARC
AGC
AHC
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "ABC\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"AGC
ABC
ARC
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "AHC\n");
    assert!(output.stderr_str().is_empty());
}
