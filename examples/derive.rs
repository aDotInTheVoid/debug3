#[derive(debug3::Debug)]
struct Foo {
    x: i32,
    y: i32,
    z: Vec<i32>,
}

fn main() {
    let f_short = Foo {
        x: 1,
        y: 2,
        z: vec![3],
    };
    debug3::dbg!(f_short);

    let f_long = Foo {
        x: 1,
        y: 2,
        z: vec![3, 4, 5, 6, 7, 8],
    };
    debug3::dbg!(f_long);
}
