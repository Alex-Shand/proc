struct Test;

impl Test {
    #[proc_function::function(crate = proc_common)]
    pub fn test(mut self) -> Result<OutputType> {
        todo!()
    }
}

fn main() {}
