struct Test;

impl Test {
    #[attribute::attribute(crate = common)]
    pub fn test(&self, item: InputType) -> Result<OutputType> {
        todo!()
    }
}

fn main() {}
