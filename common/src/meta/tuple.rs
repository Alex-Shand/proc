use super::Meta;

#[macro_use]
mod horrible_hack;

macro_rules! reverse {
    ([][$($reversed:ident),*]) => {
        ($($reversed),*)
    };
    ([$first:ident $(, $idents:ident)*][$($reversed:ident),*]) => {
        reverse!([$($idents),*][$first $(, $reversed)*])
    };
}

macro_rules! reverse_and_concat {
    ([][$($reversed:ident),*]) => {
        $($reversed)||*
    };
    ([$first:ident $(, $idents:ident)*][$($reversed:ident),*]) => {
        reverse_and_concat!([$($idents),*][$first $(, $reversed)*])
    };
}

macro_rules! expand_validate {
    ([$self:ident][][$($tt:tt)*][$($idents:ident),*]) => {
        $($tt)*
        return Ok(reverse!([$($idents),*][]));
    };
    ([$self:ident][$t:ident$(, $rest:ident)*][$($tt:tt)*][$($idents:ident),*]) => {
        expand_validate!([$self][$($rest),*][#[allow(non_snake_case)] let $t = index_tuple!($self, $t$(, $rest)*).validate()?;$($tt)*][$($idents),*])
    };
}

macro_rules! expand_parse {
    ([$self:ident, $meta:ident][][$($tt:tt)*][$($idents:ident),*]) => {
        $($tt)*
        return Ok(reverse_and_concat!([$($idents),*][]));
    };
    ([$self:ident, $meta:ident][$t:ident$(, $rest:ident)*][$($tt:tt)*][$($idents:ident),*]) => {
        expand_parse!([$self, $meta][$($rest),*][#[allow(non_snake_case)] let $t = index_tuple!($self, $t$(, $rest)*).parse_impl($meta)?;$($tt)*][$($idents),*])
    }
}

macro_rules! tuple_impl {
    ($t:ident) => {};
    ($t:ident, $($rest:ident),+) => {
        impl<$t: Meta, $($rest: Meta),*> Meta for ($t, $($rest),*) {
            type Item = ($t::Item, $($rest::Item),*);

            fn parse_impl(&mut self, meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<bool> {
                expand_parse!([self, meta][$t, $($rest),*][][$t, $($rest),*]);
            }

            fn validate(self) -> syn::Result<Self::Item> {
                expand_validate!([self][$t, $($rest),*][][$t, $($rest),*]);
            }
        }
        tuple_impl!($($rest),*);
    }
}

tuple_impl!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y,
    Z
);
