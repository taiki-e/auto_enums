use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Transpose"];

// Implementing this with `Into` requires many type annotations.
pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    {
        let generics = data.generics();
        let fields = data.fields();
        let comma = if !generics.params.empty_or_trailing() {
            TokenStream::new()
        } else {
            quote!(,)
        };
        if quote!(#generics).to_string() != quote!(<#(#fields),*#comma>).to_string() {
            Err("all fields need to be generics")?;
        }
    }

    let root = &std_root();
    let mut ts = TokenStream::new();

    ts.extend(transpose_option(data, root)?);
    ts.extend(transpose_result(data, root)?);
    ts.extend(transpose_ok(data, root)?);
    ts.extend(transpose_err(data, root)?);

    Ok(ts)
}

fn transpose_option(data: &Data, root: &TokenStream) -> Result<TokenStream> {
    let ident = data.ident();
    let fields = data.fields();
    let option = quote!(#root::option::Option);

    let mut impls = data.impl_with_capacity(1)?;

    let ty_generics = fields.iter().map(|f| quote!(#option<#f>));
    *impls.self_ty() = parse_quote!(#ident<#(#ty_generics),*>)?;

    let transpose = data
        .variants()
        .iter()
        .map(|v| quote!(#ident::#v(x) => x.map(#ident::#v)));

    impls.push_item(parse_quote! {
        #[inline]
        fn transpose(self) -> #option<#ident<#(#fields),*>> {
            match self { #(#transpose,)* }
        }
    }?);

    Ok(impls.build().into_token_stream())
}

fn transpose_result(data: &Data, root: &TokenStream) -> Result<TokenStream> {
    let ident = data.ident();
    let fields = data.fields();
    let result = quote!(#root::result::Result);

    let mut impls = data.impl_with_capacity(1)?;

    let err_fields: &Stack<_> = &(0..fields.len())
        .map(|i| {
            let id = &format!("__E{}", i);
            impls.push_generic_param(param_ident(id));
            ident_call_site(id)
        })
        .collect();

    let ty_generics = fields
        .iter()
        .zip(err_fields.iter())
        .map(|(f, ef)| quote!(#result<#f, #ef>));
    *impls.self_ty() = parse_quote!(#ident<#(#ty_generics),*>)?;

    let transpose = data
        .variants()
        .iter()
        .map(|v| quote!(#ident::#v(x) => x.map(#ident::#v).map_err(#ident::#v)));

    impls.push_item(parse_quote! {
        #[inline]
        fn transpose(self) -> #result<#ident<#(#fields),*>, #ident<#(#err_fields),*>> {
            match self { #(#transpose,)* }
        }
    }?);

    Ok(impls.build().into_token_stream())
}

fn transpose_ok(data: &Data, root: &TokenStream) -> Result<TokenStream> {
    let ident = data.ident();
    let fields = data.fields();
    let result = quote!(#root::result::Result);

    let mut impls = data.impl_with_capacity(1)?;

    impls.push_generic_param(param_ident("__E"));

    let ty_generics = fields.iter().map(|f| quote!(#result<#f, __E>));
    *impls.self_ty() = parse_quote!(#ident<#(#ty_generics),*>)?;

    let transpose = data.variants().iter().map(|v| quote!(#ident::#v(x) => x.map(#ident::#v)));
    impls.push_item(parse_quote! {
        #[inline]
        fn transpose_ok(self) -> #result<#ident<#(#fields),*>, __E> {
            match self { #(#transpose,)* }
        }
    }?);

    Ok(impls.build().into_token_stream())
}

fn transpose_err(data: &Data, root: &TokenStream) -> Result<TokenStream> {
    let ident = data.ident();
    let fields = data.fields();
    let result = quote!(#root::result::Result);

    let mut impls = data.impl_with_capacity(1)?;

    impls.push_generic_param(param_ident("__T"));

    let ty_generics = fields.iter().map(|f| quote!(#result<__T, #f>));
    *impls.self_ty() = parse_quote!(#ident<#(#ty_generics),*>)?;

    let transpose = data.variants().iter().map(|v| quote!(#ident::#v(x) => x.map_err(#ident::#v)));
    impls.push_item(parse_quote! {
        #[inline]
        fn transpose_err(self) -> #result<__T, #ident<#(#fields),*>> {
            match self { #(#transpose,)* }
        }
    }?);

    Ok(impls.build().into_token_stream())
}
