use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"Hello,World!
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "AC\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"Hello,world!
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "WA\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"Hello!World!
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "WA\n");
    assert!(output.stderr_str().is_empty());
}

