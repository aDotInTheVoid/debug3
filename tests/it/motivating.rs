use debug3::Debug;

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

#[derive(Debug)]
enum Instr {
    Push(i32),
    Load(&'static str),
    BinOp(BinOp),
    UnOp(UnOp),
}
#[derive(Debug)]
enum BinOp {
    Div,
    Mul,
    Minus,
    Pow,
    PlusMinus,
}
#[derive(Debug)]
enum UnOp {
    Minus,
    Sqrt,
}

#[test]
fn quadratic_form() {
    let instrs = vec![
        Instr::Load("b"),
        Instr::UnOp(UnOp::Minus),
        Instr::Load("b"),
        Instr::Push(2),
        Instr::BinOp(BinOp::Pow),
        Instr::Load("a"),
        Instr::Load("c"),
        Instr::Push(4),
        Instr::BinOp(BinOp::Mul),
        Instr::BinOp(BinOp::Mul),
        Instr::BinOp(BinOp::Minus),
        Instr::UnOp(UnOp::Sqrt),
        Instr::BinOp(BinOp::PlusMinus),
        Instr::Load("a"),
        Instr::Push(2),
        Instr::BinOp(BinOp::Div),
    ];
    assert_eq!(
        debug3::pprint(instrs),
        "\
[
    Load(\"b\"),
    UnOp(Minus),
    Load(\"b\"),
    Push(2),
    BinOp(Pow),
    Load(\"a\"),
    Load(\"c\"),
    Push(4),
    BinOp(Mul),
    BinOp(Mul),
    BinOp(Minus),
    UnOp(Sqrt),
    BinOp(PlusMinus),
    Load(\"a\"),
    Push(2),
    BinOp(Div),
]"
    );
}

#[derive(Debug)]
enum LinkedList<T> {
    Empty,
    Node(T, Box<LinkedList<T>>),
}

#[test]
fn deeply_nested() {
    let mut list = LinkedList::Empty;
    for i in 0..1000 {
        list = LinkedList::Node(i, Box::new(list));
    }
    let x = debug3::pprint(&list);
    assert_eq!(x, include_str!("./linked_list.txt"))
}
