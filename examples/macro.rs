use debug3::dbg;

fn main() {
    let x = [1, 2, 3, 4, 5];

    dbg!(x);
    std::dbg!(x);

    for i in 0..x.len() {
        dbg!(i, x[i]);
        std::dbg!(i, x[i]);
    }

    let a = "Anoying Aardvark";
    let b = "Beactiful Butterftly";
    let c = "Crazy Canary";
    let d = "Dumb Dingo";
    let e = "Elegant Elephant";

    dbg!(a, b, c, d, e);
    std::dbg!(a, b, c, d, e);
}
