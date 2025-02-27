#[derive(parse::Parse)]
#[parse(crate = common, __internal_proc_hack = parse)]
#[parse(foo)]
struct Test;

fn main() {}
