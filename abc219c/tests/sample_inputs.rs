use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"bacdefghijklmnopqrstuvwxzy
4
abx
bzz
bzy
caa
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        "bzz
bzy
abx
caa\n"
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"zyxwvutsrqponmlkjihgfedcba
5
a
ab
abc
ac
b
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        "b
a
ac
ab
abc\n"
    );
    assert!(output.stderr_str().is_empty());
}
