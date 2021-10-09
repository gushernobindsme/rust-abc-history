use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2 3
GCP
PPP
CCC
PPC
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3
1
2
4\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2 2
GC
PG
CG
PP
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1
2
3
4\n");
    assert!(output.stderr_str().is_empty());
}
