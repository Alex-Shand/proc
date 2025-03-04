/// Docs
#[proc_attribute::attribute(crate = proc_common, host = "foo")]
pub fn test(
    crate_: Path,
    arg1: Required<Arg1>,
    arg2: Optional<Arg2>,
    arg3: Switch,
    item: InputType,
) -> Result<OutputType> {
    todo!()
}
