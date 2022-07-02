struct NoDebug;

#[derive(debug3::Debug)]
struct HasDebug(NoDebug);

fn main() {}
