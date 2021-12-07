struct Something;

impl Something {
    #[proc::attribute]
    fn test(&self) -> proc::TokenStream {
        todo!()
    }
}

fn main() {}
