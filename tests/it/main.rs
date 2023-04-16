fn check(actual: impl debug3::Debug, expacted: expect_test::Expect) {
    expacted.assert_eq(&debug3::pprint(actual));
}

mod algo;
mod builder;
mod derive;
mod fxhash;
mod kdl;
mod motivating;
mod pulldown_cmark;
mod rustdoc_types;
mod serde_json;
mod std;
mod syn;
mod ui;
mod ungrammar;
