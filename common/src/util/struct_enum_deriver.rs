use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Generics, Ident,
    Result, Variant,
};

/// Helper struct implementing standard derive boilerplate for a derive macro
/// which can be applied to a `struct` or an `enum` but not a `union`. For
/// handling all three just use [`DeriveInput`] directly, to handle just one use
/// [`ItemStruct`](syn::ItemStruct), [`ItemUnion`](syn::ItemUnion) or
/// [`ItemEnum`](syn::ItemEnum)
#[expect(missing_debug_implementations)]
pub struct StructEnumDeriver<T>(Inner<T>);

/// Information about the struct type being derived
#[derive(Debug)]
pub struct StructData {
    #[expect(missing_docs)]
    pub ident: Ident,
    #[expect(missing_docs)]
    pub generics: Generics,
    #[expect(missing_docs)]
    pub fields: Fields,
}

/// Information about the enum type being derived
#[derive(Debug)]
pub struct EnumData {
    #[expect(missing_docs)]
    pub ident: Ident,
    #[expect(missing_docs)]
    pub generics: Generics,
    #[expect(missing_docs)]
    pub variants: Vec<Variant>,
}

impl<T> StructEnumDeriver<T> {
    /// Constructor
    ///
    /// # Errors
    /// An error is returned if the DeriveInput refers to a `union`. This error
    /// should be propagated out of the macro or handled via [`ResultFormatter`](super::ResultFormatter)
    pub fn new(
        name: &str,
        item: DeriveInput,
        extra_data: T,
        struct_logic: impl Fn(&StructData, &T, &mut TokenStream) + 'static,
        enum_logic: impl Fn(&EnumData, &T, &mut TokenStream) + 'static,
    ) -> Result<Self> {
        match item.data {
            Data::Struct(data) => {
                Ok(StructEnumDeriver(Inner::Struct(StructDeriver::new(
                    item.ident,
                    item.generics,
                    data,
                    extra_data,
                    Box::new(struct_logic),
                ))))
            }
            Data::Enum(data) => {
                Ok(StructEnumDeriver(Inner::Enum(EnumDeriver::new(
                    item.ident,
                    item.generics,
                    data,
                    extra_data,
                    Box::new(enum_logic),
                ))))
            }
            Data::Union(data) => Err(Error::new_spanned(
                data.union_token,
                format!("#[derive({name})] cannot be used on unions"),
            )),
        }
    }
}

impl<T> ToTokens for StructEnumDeriver<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

enum Inner<T> {
    Struct(StructDeriver<T>),
    Enum(EnumDeriver<T>),
}

impl<T> ToTokens for Inner<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Struct(s) => s.to_tokens(tokens),
            Self::Enum(e) => e.to_tokens(tokens),
        }
    }
}

type StructLogic<T> = Box<dyn Fn(&StructData, &T, &mut TokenStream)>;

struct StructDeriver<T> {
    data: StructData,
    extra_data: T,
    logic: StructLogic<T>,
}

impl<T> StructDeriver<T> {
    fn new(
        ident: Ident,
        generics: Generics,
        data: DataStruct,
        extra_data: T,
        logic: StructLogic<T>,
    ) -> Self {
        Self {
            data: StructData {
                ident,
                generics,
                fields: data.fields,
            },
            extra_data,
            logic,
        }
    }
}

impl<T> ToTokens for StructDeriver<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (self.logic)(&self.data, &self.extra_data, tokens);
    }
}

type EnumLogic<T> = Box<dyn Fn(&EnumData, &T, &mut TokenStream)>;

struct EnumDeriver<T> {
    data: EnumData,
    extra_data: T,
    logic: EnumLogic<T>,
}

impl<T> EnumDeriver<T> {
    fn new(
        ident: Ident,
        generics: Generics,
        data: DataEnum,
        extra_data: T,
        logic: EnumLogic<T>,
    ) -> Self {
        Self {
            data: EnumData {
                ident,
                generics,
                variants: data.variants.into_iter().collect(),
            },
            extra_data,
            logic,
        }
    }
}

impl<T> ToTokens for EnumDeriver<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (self.logic)(&self.data, &self.extra_data, tokens);
    }
}

#[cfg(test)]
mod tests {
    use pa::assert_eq;
    use quote::{quote, ToTokens};

    use super::StructEnumDeriver;

    #[test]
    fn derive_union() {
        let item: syn::DeriveInput = syn::parse_quote! {
            union MyUnion {
                a: usize,
                b: String,
            }
        };
        let result = StructEnumDeriver::new(
            "MyMacro",
            item,
            (),
            |_, (), _| unreachable!(),
            |_, (), _| unreachable!(),
        );
        assert!(result.is_err());
        let Err(error) = result else { unreachable!() };
        assert_eq!(
            r##":: core :: compile_error ! { "#[derive(MyMacro)] cannot be used on unions" }"##,
            error.into_compile_error().to_string()
        );
    }

    #[test]
    fn derive_struct() -> syn::Result<()> {
        let item: syn::DeriveInput = syn::parse_quote! {
            struct MyStruct<Generic> {
                a: usize,
                b: String,
            }
        };
        let result = StructEnumDeriver::new(
            "MyMacro",
            item,
            (),
            |data, (), tokens| {
                assert_eq!("MyStruct", data.ident.to_string());
                assert_eq!(1, data.generics.params.len());
                assert_eq!(2, data.fields.len());
                tokens.extend(quote! { Tokens go "here" });
            },
            |_, (), _| unreachable!(),
        )?;

        assert_eq!(
            r#"Tokens go "here""#,
            result.into_token_stream().to_string()
        );

        Ok(())
    }

    #[test]
    fn derive_enum() -> syn::Result<()> {
        let item: syn::DeriveInput = syn::parse_quote! {
            enum MyEnum<Generic> {
                A(usize),
                B(String),
            }
        };
        let result = StructEnumDeriver::new(
            "MyMacro",
            item,
            (),
            |_, (), _| unreachable!(),
            |data, (), tokens| {
                assert_eq!("MyEnum", data.ident.to_string());
                assert_eq!(1, data.generics.params.len());
                assert_eq!(2, data.variants.len());
                tokens.extend(quote! { Tokens go "here" });
            },
        )?;

        assert_eq!(
            r#"Tokens go "here""#,
            result.into_token_stream().to_string()
        );

        Ok(())
    }
}
