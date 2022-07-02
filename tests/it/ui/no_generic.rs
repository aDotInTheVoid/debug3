use debug3::{pprint, Debug};

#[derive(Debug)]
struct Triple<X, Y, Z> {
    x: X,
    y: Y,
    z: Z,
}

struct NoDebug;

fn main() {
    pprint(Triple {
        x: 1u8,
        y: 2u16,
        z: NoDebug,
    });
}
