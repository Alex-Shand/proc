struct Test;

impl Test {
    #[attribute::attribute(crate = common)]
    pub fn test(&mut self, item: InputType) -> Result<OutputType> {
        todo!()
    }
}

fn main() {}
