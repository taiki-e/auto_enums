use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Generics;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Transpose"];

// Implementing this with `Into` requires many type annotations.

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    let root = &std_root();
    let generics = &data.generics;
    let mut ts = TokenStream::new();

    ts.extend(transpose_option(
        &EnumData::parse(data, false, false)?,
        generics,
        root,
    )?);
    EnumData::parse(data, true, false).map(|data| {
        ts.extend(transpose_result(&data, root));
        ts.extend(transpose_ok(&data, root));
        ts.extend(transpose_err(&data, root));
        ts
    })
}

fn ident_call_site(s: &str) -> Ident {
    Ident::new(s, Span::call_site())
}

fn transpose_option(
    data: &EnumData<'_>,
    generics: &Generics,
    root: &TokenStream,
) -> Result<TokenStream> {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let comma = if !generics.params.empty_or_trailing() {
        TokenStream::new()
    } else {
        quote!(,)
    };
    if quote!(#ty_generics).to_string() != quote!(<#(#fields),*#comma>).to_string() {
        Err("all fields need to be generics")?;
    }

    let option = quote!(#root::option::Option);
    let ty_generics = fields.iter().fold(TokenStream::new(), |t, f| {
        t.extend_and_return(quote!(#option<#f>,))
    });

    // method
    let transpose = variants.iter().fold(TokenStream::new(), |t, v| {
        t.extend_and_return(quote!(#v(x) => x.map(#v),))
    });

    Ok(quote! {
        impl #impl_generics #name<#ty_generics> #where_clause {
            #[inline]
            fn transpose(self) -> #option<#name<#(#fields),*>> {
                match self { #transpose }
            }
        }
    })
}

fn transpose_result(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        where_clause,
        variants,
        fields,
        ..
    } = data;

    let result = quote!(#root::result::Result);
    let err_fields: &Stack<_> = &(0..fields.len())
        .map(|i| ident_call_site(&format!("__E{}", i)))
        .collect();

    let impl_generics = quote!(#impl_generics #(#err_fields),*>);
    let ty_generics = fields
        .iter()
        .zip(err_fields.iter())
        .fold(TokenStream::new(), |t, (f, ef)| {
            t.extend_and_return(quote!(#result<#f, #ef>,))
        });

    // method
    let transpose = variants.iter().fold(TokenStream::new(), |t, v| {
        t.extend_and_return(quote!(#v(x) => x.map(#v).map_err(#v),))
    });

    quote! {
        impl #impl_generics #name<#ty_generics> #where_clause {
            #[inline]
            fn transpose(self) -> #result<#name<#(#fields),*>, #name<#(#err_fields),*>> {
                match self { #transpose }
            }
        }
    }
}

fn transpose_ok(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        where_clause,
        variants,
        fields,
        ..
    } = data;

    let result = quote!(#root::result::Result);
    let impl_generics = quote!(#impl_generics __E>);
    let ty_generics = fields.iter().fold(TokenStream::new(), |t, f| {
        t.extend_and_return(quote!(#result<#f, __E>,))
    });

    // method
    let transpose = variants.iter().fold(TokenStream::new(), |t, v| {
        t.extend_and_return(quote!(#v(x) => x.map(#v),))
    });

    quote! {
        impl #impl_generics #name<#ty_generics> #where_clause {
            #[inline]
            fn transpose_ok(self) -> #result<#name<#(#fields),*>, __E> {
                match self { #transpose }
            }
        }
    }
}

fn transpose_err(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        where_clause,
        variants,
        fields,
        ..
    } = data;

    let result = quote!(#root::result::Result);
    let impl_generics = quote!(#impl_generics __T>);
    let ty_generics = fields.iter().fold(TokenStream::new(), |t, f| {
        t.extend_and_return(quote!(#result<__T, #f>,))
    });

    // method
    let transpose = variants.iter().fold(TokenStream::new(), |t, v| {
        t.extend_and_return(quote!(#v(x) => x.map_err(#v),))
    });

    quote! {
        impl #impl_generics #name<#ty_generics> #where_clause {
            #[inline]
            fn transpose_err(self) -> #result<__T, #name<#(#fields),*>> {
                match self { #transpose }
            }
        }
    }
}
