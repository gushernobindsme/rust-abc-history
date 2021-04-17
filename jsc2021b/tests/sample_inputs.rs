use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2 2
1 2
1 3
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2 3\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"4 4
1 2 3 4
1 2 3 4
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "\n");
    assert!(output.stderr_str().is_empty());
}
