use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use proc_macro2::TokenTree;

#[derive(Debug)]
pub struct TokenStream(pub proc_macro2::TokenStream);

impl PartialEq for TokenStream {
    fn eq(&self, rhs: &Self) -> bool {
        are_equal(self.0.clone(), rhs.0.clone())
    }
}

fn are_equal(lhs: proc_macro2::TokenStream, rhs: proc_macro2::TokenStream) -> bool {
    let lhs = lhs.into_iter().collect::<Vec<_>>();
    let rhs = rhs.into_iter().collect::<Vec<_>>();
    if lhs.len() != rhs.len() {
        return false;
    }

    lhs.into_iter().zip(rhs).all(|(l, r)| is_equal(l, r))
}

fn is_equal(l: TokenTree, r: TokenTree) -> bool {
    match (l, r) {
        (TokenTree::Punct(pl), TokenTree::Punct(pr)) => pl.as_char() == pr.as_char(),
        (TokenTree::Group(gl), TokenTree::Group(gr)) => are_equal(gl.stream(), gr.stream()),
        (TokenTree::Ident(il), TokenTree::Ident(ir)) => il == ir,
        (TokenTree::Literal(ll), TokenTree::Literal(lr)) => ll.to_string() == lr.to_string(),
        _ => false,
    }
}

#[doc(hidden)]
pub fn check_expansion<
    P: AsRef<Path>,
    F: Fn(proc_macro2::TokenStream) -> Result<TokenStream, String>,
>(
    path: P,
    run_macro: F,
) {
    let mut tests: HashMap<String, (Option<PathBuf>, Option<PathBuf>)> = HashMap::new();
    for file in path.as_ref().read_dir().unwrap() {
        let path = file.unwrap().path();
        if let Some(ext) = path.extension() {
            if ext != "rs" {
                continue;
            }
        } else {
            continue;
        }
        let stem = path.file_stem().unwrap().to_str().unwrap();
        let (key, is_expanded) = if stem.ends_with(".expanded") {
            (stem.rsplit_once('.').unwrap().0, true)
        } else {
            (stem, false)
        };
        let entry = tests.entry(key.to_owned()).or_insert((None, None));
        if is_expanded {
            entry.1 = Some(path);
        } else {
            entry.0 = Some(path);
        }
    }
    for test in tests {
        let (_name, (input, output)) = test;
        let input = read_tokens(input.unwrap());
        let output = TokenStream(read_tokens(output.unwrap()));
        assert_eq!(run_macro(input).unwrap(), output);
    }
}

fn read_tokens(path: PathBuf) -> proc_macro2::TokenStream {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .skip_while(|line| *line != "//PROC: Start")
        .take_while(|line| *line != "//PROC: End")
        .collect::<Vec<_>>()
        .join("\n")
        .parse()
        .unwrap()
}
