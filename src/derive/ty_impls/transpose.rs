// SPDX-License-Identifier: Apache-2.0 OR MIT

use derive_utils::EnumImpl;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::TypeParam;

use crate::derive::*;

pub(crate) const NAME: &[&str] = &["Transpose"];

// Implementing this with `Into` requires many type annotations.
pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
    check_fields(data)?;
    let mut items = TokenStream::new();
    items.extend(transpose_option(data));
    items.extend(transpose_result(data));
    items.extend(transpose_ok(data));
    items.extend(transpose_err(data));
    Ok(items)
}

fn check_fields(data: &Data) -> Result<()> {
    let generics = &data.generics;
    let fields = data.field_types();
    let comma = if generics.params.empty_or_trailing() { quote!(,) } else { TokenStream::new() };

    if quote!(#generics).to_string() == quote!(<#(#fields),*#comma>).to_string() {
        Ok(())
    } else {
        bail!(data, "all fields need to be generics")
    }
}

fn transpose_option(data: &Data) -> TokenStream {
    let ident = &data.ident;
    let mut impl_ = EnumImpl::new(data);

    let transpose = data.variant_idents().map(|v| quote!(#ident::#v(x) => x.map(#ident::#v)));

    let fields = data.field_types();
    impl_.push_item(parse_quote! {
        #[inline]
        fn transpose(self) -> ::core::option::Option<#ident<#(#fields),*>> {
            match self { #(#transpose,)* }
        }
    });

    let mut impl_ = impl_.build_impl();

    let ty_generics = data.field_types().map(|f| quote!(::core::option::Option<#f>));
    impl_.self_ty = parse_quote!(#ident<#(#ty_generics),*>);
    impl_.into_token_stream()
}

fn transpose_result(data: &Data) -> TokenStream {
    let ident = &data.ident;
    let fields = data.field_types();
    let mut impl_ = EnumImpl::new(data);

    let err_fields: Vec<_> = (0..fields.len())
        .map(|i| {
            let id = format_ident!("__E{}", i);
            impl_.push_generic_param(TypeParam::from(id.clone()).into());
            id
        })
        .collect();

    let transpose = data
        .variant_idents()
        .map(|v| quote!(#ident::#v(x) => x.map(#ident::#v).map_err(#ident::#v)));
    impl_.push_item(parse_quote! {
        #[inline]
        fn transpose(
            self,
        ) -> ::core::result::Result<#ident<#(#fields),*>, #ident<#(#err_fields),*>> {
            match self { #(#transpose,)* }
        }
    });

    let mut impl_ = impl_.build_impl();

    let ty_generics = data
        .field_types()
        .zip(err_fields.iter())
        .map(|(f, ef)| quote!(::core::result::Result<#f, #ef>));
    impl_.self_ty = parse_quote!(#ident<#(#ty_generics),*>);
    impl_.into_token_stream()
}

fn transpose_ok(data: &Data) -> TokenStream {
    let ident = &data.ident;
    let fields = data.field_types();
    let mut impl_ = EnumImpl::new(data);

    impl_.push_generic_param(TypeParam::from(format_ident!("__E")).into());

    let transpose = data.variant_idents().map(|v| quote!(#ident::#v(x) => x.map(#ident::#v)));
    impl_.push_item(parse_quote! {
        #[inline]
        fn transpose_ok(self) -> ::core::result::Result<#ident<#(#fields),*>, __E> {
            match self { #(#transpose,)* }
        }
    });

    let mut impl_ = impl_.build_impl();

    let ty_generics = data.field_types().map(|f| quote!(::core::result::Result<#f, __E>));
    impl_.self_ty = parse_quote!(#ident<#(#ty_generics),*>);
    impl_.into_token_stream()
}

fn transpose_err(data: &Data) -> TokenStream {
    let ident = &data.ident;
    let fields = data.field_types();
    let mut impl_ = EnumImpl::new(data);

    impl_.push_generic_param(TypeParam::from(format_ident!("__T")).into());

    let transpose = data.variant_idents().map(|v| quote!(#ident::#v(x) => x.map_err(#ident::#v)));
    impl_.push_item(parse_quote! {
        #[inline]
        fn transpose_err(self) -> ::core::result::Result<__T, #ident<#(#fields),*>> {
            match self { #(#transpose,)* }
        }
    });

    let mut impl_ = impl_.build_impl();

    let ty_generics = data.field_types().map(|f| quote!(::core::result::Result<__T, #f>));
    impl_.self_ty = parse_quote!(#ident<#(#ty_generics),*>);
    impl_.into_token_stream()
}
