use debug3::{pprint, Debug};
use expect_test::{expect, Expect};
use serde_json::json;

fn check(actual: impl Debug, expacted: Expect) {
    expacted.assert_eq(&pprint(actual));
}

#[test]
fn basic() {
    check(
        json!({ "a": 64, "b": i64::MAX as u64 + 200, "c": 3.14159  }),
        expect![[r#"
            Object {
                a: Number(64),
                b: Number(9223372036854776007),
                c: Number(3.14159),
            }"#]],
    )
}

#[test]
fn phonebook() {
    check(
        json!({
            "name": "John Doe",
            "age": 43,
            "address": {
                "street": "10 Downing Street",
                "city": "London"
            },
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }),
        expect![[r#"
            Object {
                address: Object {
                    city: String("London"),
                    street: String("10 Downing Street"),
                },
                age: Number(43),
                name: String("John Doe"),
                phones: Array [String("+44 1234567"), String("+44 2345678")],
            }"#]],
    )
}
