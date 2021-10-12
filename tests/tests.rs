use trybuild::TestCases;

#[test]
fn test_result_macro() {
    let tests = TestCases::new();

    tests.pass("tests/result_tests/*.rs");
}