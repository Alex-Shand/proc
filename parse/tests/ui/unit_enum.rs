#[derive(parse::Parse)]
#[parse(crate = common, __internal_proc_hack = parse)]
enum Unit {
    Unit,
}

fn main() {}
