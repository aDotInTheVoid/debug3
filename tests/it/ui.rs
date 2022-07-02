// #[rustversion::attr(not(nightly), ignore)]
#[test]
#[ignore = "https://github.com/dtolnay/trybuild/issues/171"]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
