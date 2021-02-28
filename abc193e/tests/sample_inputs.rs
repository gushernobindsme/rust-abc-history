use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3
5 2 7 6
1 1 3 1
999999999 1 1000000000 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        "20
infinity
1000000000999999999\n"
    );
    assert!(output.stderr_str().is_empty());
}
