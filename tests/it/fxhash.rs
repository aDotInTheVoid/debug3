use crate::check;
use expect_test::expect;

#[test]
fn can_use_custom_map() {
    let mut map = fxhash::FxHashMap::default();
    check(&map, expect![[r#"{}"#]]);
    map.insert("a", 1);
    check(&map, expect![[r#"{"a": 1}"#]]);
    map.insert("b", 2);
    // FxHash map is not randomized
    check(&map, expect![[r#"{"b": 2, "a": 1}"#]]);
}

#[test]
fn can_use_custom_hash_set() {
    let mut set = fxhash::FxHashSet::default();
    check(&set, expect![[r#"{}"#]]);
    set.insert(1);
    check(&set, expect![[r#"{1}"#]]);
    set.insert(2);
    // FxHash isn't randomized
    check(&set, expect![[r#"{1, 2}"#]]);
}
