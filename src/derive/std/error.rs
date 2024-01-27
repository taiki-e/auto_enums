// SPDX-License-Identifier: Apache-2.0 OR MIT

use derive_utils::EnumImpl;

use crate::derive::*;

pub(crate) const NAME: &[&str] = &["Error"];

pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
    let ident = &data.ident;
    let source =
        data.variant_idents().map(|v| quote!(#ident::#v(x) => ::std::option::Option::Some(x)));

    let source = parse_quote! {
        fn source(&self) -> ::std::option::Option<&(dyn (::std::error::Error) + 'static)> {
            match self { #(#source,)* }
        }
    };

    let mut impl_ =
        EnumImpl::from_trait(data, &parse_quote!(::std::error::Error), None, parse_quote! {
            trait Error {
                #[allow(deprecated)]
                fn description(&self) -> &str;
            }
        });

    data.field_types().for_each(|f| impl_.push_where_predicate(parse_quote!(#f: 'static)));
    impl_.push_item(source);

    Ok(impl_.build())
}
