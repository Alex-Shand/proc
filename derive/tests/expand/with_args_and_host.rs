/// Docs
#[proc_derive::derive(crate = proc_common, host = "foo", name = MyDerive)]
pub fn derive(
    crate_: Path,
    arg1: Required<Arg1>,
    arg2: Optional<Arg2>,
    arg3: Switch,
    item: InputType,
) -> Result<OutputType> {
    todo!()
}
