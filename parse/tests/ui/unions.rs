#[derive(proc_parse::Parse)]
#[parse(crate = proc_common, __internal_proc_hack = proc_parse)]
union Union {
    a: u32,
}

fn main() {}
