struct Something;

impl Something {
    #[proc::attribute]
    fn test(&mut self) -> proc::TokenStream {
        todo!()
    }
}

fn main() {}
