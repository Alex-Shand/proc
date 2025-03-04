/// Docs
#[proc_attribute::attribute(crate = proc_common, host = "foo")]
pub fn test(crate_: Path, item: InputType) -> Result<OutputType> {
    todo!()
}
