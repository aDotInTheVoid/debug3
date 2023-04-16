#![cfg(feature = "pulldown-cmark")]

use expect_test::{expect, Expect};

fn check(input: &str, expected: Expect) {
    let p = pulldown_cmark::Parser::new(input);
    let events: Vec<_> = p.collect();
    crate::check(events, expected);
}

#[test]
fn basic() {
    check(
        r"
# Some markdown document

**Bold** _italic_ `code` [link](https://example.com)
",
        expect![[r#"
            [
                Start(Heading(H1, None, [])),
                Text("Some markdown document"),
                End(Heading(H1, None, [])),
                Start(Paragraph),
                Start(Strong),
                Text("Bold"),
                End(Strong),
                Text(" "),
                Start(Emphasis),
                Text("italic"),
                End(Emphasis),
                Text(" "),
                Code("code"),
                Text(" "),
                Start(Link(Inline, "https://example.com", "")),
                Text("link"),
                End(Link(Inline, "https://example.com", "")),
                End(Paragraph),
            ]"#]],
    )
}
