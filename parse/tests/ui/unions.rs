#[derive(parse::Parse)]
#[parse(crate = common, __internal_proc_hack = parse)]
union Union {
    a: u32,
}

fn main() {}
