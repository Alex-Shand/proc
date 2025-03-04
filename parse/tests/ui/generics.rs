#[derive(proc_parse::Parse)]
#[parse(crate = proc_common, __internal_proc_hack = proc_parse)]
struct WithGenerics<T>(T);

fn main() {}
