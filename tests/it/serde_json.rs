#![cfg(feature = "serde_json")]

use crate::check;
use expect_test::expect;
use serde_json::json;

#[test]
fn basic() {
    check(
        json!({ "a": 64, "b": i64::MAX as u64 + 200, "c": 12.345  }),
        expect![[r#"
            Object {
                a: Number(64),
                b: Number(9223372036854776007),
                c: Number(12.345),
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
