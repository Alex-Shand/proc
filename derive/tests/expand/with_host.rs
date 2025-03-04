/// Docs
#[proc_derive::derive(crate = proc_common, host = "foo", name = MyDerive)]
pub fn derive(crate_: Path, item: InputType) -> Result<OutputType> {
    todo!()
}
