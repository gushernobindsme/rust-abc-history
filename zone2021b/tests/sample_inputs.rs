use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1 10 10
3 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2.857142857142857\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1 10 10
3 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 896 483
228 59
529 310
339 60
78 266
659 391
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "245.3080684596577\n");
    assert!(output.stderr_str().is_empty());
}

