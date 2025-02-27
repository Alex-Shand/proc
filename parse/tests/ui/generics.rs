#[derive(parse::Parse)]
#[parse(crate = common, __internal_proc_hack = parse)]
struct WithGenerics<T>(T);

fn main() {}
