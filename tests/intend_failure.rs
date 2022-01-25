#[ignore]
#[test]
fn invalid_macros() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/intend_failure/*.rs");
}
