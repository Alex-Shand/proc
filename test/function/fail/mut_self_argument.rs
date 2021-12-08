struct Something;

impl Something {
    #[proc::function]
    fn test(&mut self) -> proc::TokenStream {
        todo!()
    }
}

fn main() {}
