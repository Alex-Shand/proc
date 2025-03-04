struct Test;

impl Test {
    #[proc_attribute::attribute(crate = proc_common)]
    pub fn test(&mut self, item: InputType) -> Result<OutputType> {
        todo!()
    }
}

fn main() {}
