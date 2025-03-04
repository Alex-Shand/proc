struct Test;

impl Test {
    #[proc_derive::derive(crate = proc_common, name = MyDerive)]
    pub fn test(&self, item: InputType) -> Result<OutputType> {
        todo!()
    }
}

fn main() {}
