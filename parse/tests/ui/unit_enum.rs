#[derive(proc_parse::Parse)]
#[parse(crate = proc_common, __internal_proc_hack = proc_parse)]
enum Unit {
    Unit,
}

fn main() {}
