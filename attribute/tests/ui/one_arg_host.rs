#[proc_attribute::attribute(crate = proc_common, host = "foo")]
pub fn test(item: InputType) -> Result<OutputType> {
    todo!()
}

fn main() {}
