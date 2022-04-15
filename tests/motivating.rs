#[test]
fn motivating() {
    let complex_structure = vec![
        vec![None, Some(2)],
        vec![Some(2), None],
        vec![Some(4), Some(777)],
        vec![None, Some(2)],
        vec![Some(2), None],
        vec![None, None, None, None, None],
    ];

    assert_eq!(
        "[[None, Some(2)], [Some(2), None], [Some(4), Some(777)], [None, Some(2)], [Some(2), None], [None, None, None, None, None]]",
        format!("{:?}", complex_structure)
    );
    assert_eq!(
        "\
[
    [
        None,
        Some(
            2,
        ),
    ],
    [
        Some(
            2,
        ),
        None,
    ],
    [
        Some(
            4,
        ),
        Some(
            777,
        ),
    ],
    [
        None,
        Some(
            2,
        ),
    ],
    [
        Some(
            2,
        ),
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
    ],
]",
        format!("{:#?}", complex_structure)
    );
    assert_eq!(
        "\
[
    [None, Some(2)],
    [Some(2), None],
    [Some(4), Some(777)],
    [None, Some(2)],
    [Some(2), None],
    [None, None, None, None, None],
]",
        debug3::pprint(complex_structure)
    );
}
