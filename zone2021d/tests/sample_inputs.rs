use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"ozRnonnoe
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "zone\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"hellospaceRhellospace
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "\n");
    assert!(output.stderr_str().is_empty());
}
