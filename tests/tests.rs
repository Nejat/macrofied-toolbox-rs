use trybuild::TestCases;

#[test]
fn test_result_macro() {
    let tests = TestCases::new();

    tests.compile_fail("tests/result_compile_failed/*.rs");
    tests.compile_fail("tests/option_compile_failed/*.rs");
}