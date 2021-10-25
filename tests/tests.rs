use trybuild::TestCases;

#[test]
fn test_result_macro() {
    let tests = TestCases::new();

    tests.compile_fail("tests/result_compile_failed/*.rs");
}