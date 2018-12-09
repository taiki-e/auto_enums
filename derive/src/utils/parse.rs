use proc_macro2::{Ident, Punct, Spacing, TokenStream, TokenTree};
use quote::quote;
use syn::{punctuated::Pair, Fields, Generics, ItemEnum, Type, TypeGenerics, WhereClause};

use crate::utils::*;

fn parse_generics(
    generics: &Generics,
    extensible_impl_generics: bool,
    extensible_where_clause: bool,
) -> Result<(TokenStream, TypeGenerics<'_>, TokenStream)> {
    fn to_extensible_impl_generics(mut ts: Stack<TokenTree>) -> Result<TokenStream> {
        fn punct(c: char) -> Punct {
            Punct::new(c, Spacing::Alone)
        }

        match ts.len() {
            0 => ts.push(punct('<').into()),
            1 => Err("invalid generics")?,
            2 => {
                ts.pop();
            }
            _ => {
                ts.pop();
                match ts.pop().unwrap() {
                    TokenTree::Punct(ref p) if p.as_char() == ',' => {}
                    tt => ts.push(tt),
                }
                ts.push(punct(',').into());
            }
        }
        Ok(ts.into_iter().collect())
    }

    fn to_extensible_where_clause(where_clause: Option<&WhereClause>) -> TokenStream {
        match where_clause.and_then(|w| w.predicates.last()) {
            Some(Pair::Punctuated(_, _)) => quote!(#where_clause),
            Some(Pair::End(_)) => quote!(#where_clause,),
            None => quote!(where),
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let impl_generics = if extensible_impl_generics {
        to_extensible_impl_generics(quote!(#impl_generics).into_iter().collect())?
    } else {
        quote!(#impl_generics)
    };
    let where_clause = if extensible_where_clause {
        to_extensible_where_clause(where_clause)
    } else {
        quote!(#where_clause)
    };

    Ok((impl_generics, ty_generics, where_clause))
}

pub(crate) struct EnumData<'a> {
    pub(crate) name: &'a Ident,
    pub(crate) impl_generics: TokenStream,
    pub(crate) ty_generics: TypeGenerics<'a>,
    pub(crate) where_clause: TokenStream,
    pub(crate) variants: Stack<TokenStream>,
    pub(crate) fields: Stack<&'a Type>,
}

impl<'a> EnumData<'a> {
    pub(crate) fn parse(
        data: &'a ItemEnum,
        extensible_impl_generics: bool,
        extensible_where_clause: bool,
    ) -> Result<Self> {
        let len = data.variants.len();
        if len < 2 {
            Err("cannot be implemented for enums with less than two variants")?;
        }

        let name = &data.ident;
        let mut variants = Stack::with_capacity(len);
        let mut fields = Stack::with_capacity(len);
        for v in &data.variants {
            if v.discriminant.is_some() {
                Err("cannot be implemented for enums with discriminants")?;
            }

            match &v.fields {
                Fields::Unnamed(f) => match f.unnamed.len() {
                    1 => fields.push(&f.unnamed.iter().next().unwrap().ty),
                    0 => Err("cannot be implemented for enums with zero fields")?,
                    _ => Err("cannot be implemented for enums with multiple fields")?,
                },
                Fields::Unit => Err("cannot be implemented for enums with units")?,
                Fields::Named(_) => Err("cannot be implemented for enums with named fields")?,
            }

            let v = &v.ident;
            variants.push(quote!(#name::#v));
        }

        let (impl_generics, ty_generics, where_clause) = parse_generics(
            &data.generics,
            extensible_impl_generics,
            extensible_where_clause,
        )?;

        Ok(EnumData {
            name,
            impl_generics,
            ty_generics,
            where_clause,
            variants,
            fields,
        })
    }
}
