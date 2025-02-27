struct Test;

impl Test {
    #[derive::derive(crate = common, name = MyDerive)]
    pub fn test(mut self, item: InputType) -> Result<OutputType> {
        todo!()
    }
}

fn main() {}
