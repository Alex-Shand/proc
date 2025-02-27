struct Test;

impl Test {
    #[derive::derive(crate = common, name = MyDerive)]
    pub fn test(&self, item: InputType) -> Result<OutputType> {
        todo!()
    }
}

fn main() {}
