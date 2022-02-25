#[test]
#[rustversion::attr(not(nightly), ignore)]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
