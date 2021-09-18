use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"mari
to
zzo
1321
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "marizzotomari\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"abra
cad
abra
123
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "abracadabra\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"a
b
c
1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "a\n");
    assert!(output.stderr_str().is_empty());
}
