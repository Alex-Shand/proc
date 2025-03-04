#[derive(proc_parse::Parse)]
#[parse(crate = proc_common, __internal_proc_hack = proc_parse)]
#[parse(foo)]
struct Test;

fn main() {}
