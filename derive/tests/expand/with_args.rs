/// Docs
#[derive::derive(crate = common, name = MyDerive)]
pub fn derive(
    arg1: Required<Arg1>,
    arg2: Optional<Arg2>,
    arg3: Switch,
    item: InputType,
) -> Result<OutputType> {
    todo!()
}
