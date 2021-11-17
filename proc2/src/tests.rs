use std::collections::HashMap;
use std::path::Path;

use anyhow::{anyhow, Context, Result};

use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    token::Macro,
    Ident, LitStr, Token,
};

proc_common::keywords! {pass, fail}

pub(super) struct Input {
    pub(super) name: Ident,
    pub(super) pass: LitStr,
    pub(super) fail: LitStr,
}

impl Parse for Input {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let name;
        let pass;
        let fail;
        if input.peek(Macro) {
            name = parse::<Macro, _>(input)?;
            if input.peek(kw::pass) {
                pass = parse::<kw::pass, _>(input)?;
                fail = parse::<kw::fail, _>(input)?;
            } else {
                fail = parse::<kw::fail, _>(input)?;
                pass = parse::<kw::pass, _>(input)?;
            }
        } else if input.peek(kw::pass) {
            pass = parse::<kw::pass, _>(input)?;
            if input.peek(Macro) {
                name = parse::<Macro, _>(input)?;
                fail = parse::<kw::fail, _>(input)?;
            } else {
                fail = parse::<kw::fail, _>(input)?;
                name = parse::<Macro, _>(input)?;
            }
        } else {
            fail = parse::<kw::fail, _>(input)?;
            if input.peek(Macro) {
                name = parse::<Macro, _>(input)?;
                pass = parse::<kw::pass, _>(input)?;
            } else {
                pass = parse::<kw::pass, _>(input)?;
                name = parse::<Macro, _>(input)?;
            }
        }
        Ok(Input { name, pass, fail })
    }
}

fn parse<KW, R>(input: ParseStream<'_>) -> syn::Result<R>
where
    KW: Parse,
    R: Parse,
{
    drop(input.parse::<KW>()?);
    let _ = input.parse::<Token![=]>()?;
    let data = input.parse::<R>()?;
    let _ = input.parse::<Option<Token![;]>>()?;
    Ok(data)
}

pub(super) fn get_expansion_tests(path: &Path) -> Result<Vec<(Ident, String, String)>> {
    let listdir_error = || {
        format!(
            "Failed to list the contents of the directory '{}'",
            path.display()
        )
    };

    let missing_file_error = |name, ftype| {
        anyhow!(
            "Test {} has no {} file (Expected to find '{}/{}.rs')",
            name,
            ftype,
            path.display(),
            name
        )
    };

    let utf8_error = |file: &_| anyhow!("Filename '{:?}' is not valid utf-8", file);

    let mut tests = HashMap::new();
    let files = path.read_dir().with_context(listdir_error)?;
    for file in files {
        let file = file.with_context(listdir_error)?.path();
        if file.is_dir() {
            continue;
        }
        if let Some(ext) = file.extension() {
            if ext != "rs" {
                continue;
            }
            let stem = file
                .file_stem()
                .ok_or_else(|| anyhow!("File '{}' turned into a directory", file.display()))?
                .to_str()
                .ok_or_else(|| utf8_error(file.as_os_str()))?;
            let (key, is_expanded) = if stem.ends_with(".expanded") {
                (
                    stem.rsplit_once('.')
                        .expect("Checked for a '.' then didn't find it")
                        .0,
                    true,
                )
            } else {
                (stem, false)
            };
            let key = key.to_owned();

            let file = file
                .into_os_string()
                .into_string()
                .map_err(|file| utf8_error(&file))?;
            let entry = tests.entry(key).or_insert((None, None));
            if is_expanded {
                entry.1 = Some(file);
            } else {
                entry.0 = Some(file);
            }
        }
    }

    let mut completed_tests = Vec::new();
    for (name, (input, output)) in tests {
        let input = input.ok_or_else(|| missing_file_error(name.clone(), "input"))?;
        let output =
            output.ok_or_else(|| missing_file_error(format!("{}.expanded", name), "output"))?;
        completed_tests.push((Ident::new(&name, Span::call_site()), input, output));
    }
    Ok(completed_tests)
}
