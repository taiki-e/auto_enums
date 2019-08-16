use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Transpose"];

// Implementing this with `Into` requires many type annotations.
pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    check_fields(data)?;

    items.push(transpose_option(data)?);
    items.push(transpose_result(data)?);
    items.push(transpose_ok(data)?);
    items.push(transpose_err(data)?);

    Ok(())
}

fn check_fields(data: &Data) -> Result<()> {
    let generics = data.generics();
    let fields = data.fields();
    let comma = if generics.params.empty_or_trailing() { quote!(,) } else { TokenStream::new() };

    if quote!(#generics).to_string() == quote!(<#(#fields),*#comma>).to_string() {
        Ok(())
    } else {
        Err(error!(data.span, "all fields need to be generics"))
    }
}

fn transpose_option(data: &Data) -> Result<ItemImpl> {
    let ident = data.ident();
    let fields = data.fields();

    let mut items = data.impl_with_capacity(1)?;

    let ty_generics = fields.iter().map(|f| quote!(::core::option::Option<#f>));
    *items.self_ty() = parse_quote!(#ident<#(#ty_generics),*>)?;

    let transpose = data.variants().iter().map(|v| quote!(#ident::#v(x) => x.map(#ident::#v)));

    items.push_item(parse_quote! {
        #[inline]
        fn transpose(self) -> ::core::option::Option<#ident<#(#fields),*>> {
            match self { #(#transpose,)* }
        }
    }?);

    Ok(items.build_item())
}

fn transpose_result(data: &Data) -> Result<ItemImpl> {
    let fields = data.fields();

    let mut items = data.impl_with_capacity(1)?;

    let err_fields: &Vec<_> = &(0..fields.len())
        .map(|i| {
            let id = format_ident!("__E{}", i);
            items.push_generic_param(param_ident!("{}", id));
            id
        })
        .collect();

    let ident = data.ident();
    let ty_generics =
        fields.iter().zip(err_fields.iter()).map(|(f, ef)| quote!(::core::result::Result<#f, #ef>));
    *items.self_ty() = parse_quote!(#ident<#(#ty_generics),*>)?;

    let transpose = data
        .variants()
        .iter()
        .map(|v| quote!(#ident::#v(x) => x.map(#ident::#v).map_err(#ident::#v)));

    items.push_item(parse_quote! {
        #[inline]
        fn transpose(self) -> ::core::result::Result<#ident<#(#fields),*>, #ident<#(#err_fields),*>> {
            match self { #(#transpose,)* }
        }
    }?);

    Ok(items.build_item())
}

fn transpose_ok(data: &Data) -> Result<ItemImpl> {
    let ident = data.ident();
    let fields = data.fields();

    let mut items = data.impl_with_capacity(1)?;

    items.push_generic_param(param_ident!("__E"));

    let ty_generics = fields.iter().map(|f| quote!(::core::result::Result<#f, __E>));
    *items.self_ty() = parse_quote!(#ident<#(#ty_generics),*>)?;

    let transpose = data.variants().iter().map(|v| quote!(#ident::#v(x) => x.map(#ident::#v)));
    items.push_item(parse_quote! {
        #[inline]
        fn transpose_ok(self) -> ::core::result::Result<#ident<#(#fields),*>, __E> {
            match self { #(#transpose,)* }
        }
    }?);

    Ok(items.build_item())
}

fn transpose_err(data: &Data) -> Result<ItemImpl> {
    let ident = data.ident();
    let fields = data.fields();

    let mut items = data.impl_with_capacity(1)?;

    items.push_generic_param(param_ident!("__T"));

    let ty_generics = fields.iter().map(|f| quote!(::core::result::Result<__T, #f>));
    *items.self_ty() = parse_quote!(#ident<#(#ty_generics),*>)?;

    let transpose = data.variants().iter().map(|v| quote!(#ident::#v(x) => x.map_err(#ident::#v)));
    items.push_item(parse_quote! {
        #[inline]
        fn transpose_err(self) -> ::core::result::Result<__T, #ident<#(#fields),*>> {
            match self { #(#transpose,)* }
        }
    }?);

    Ok(items.build_item())
}
