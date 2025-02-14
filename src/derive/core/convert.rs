// SPDX-License-Identifier: Apache-2.0 OR MIT

pub(crate) mod as_ref {
    use crate::derive::prelude::*;

    pub(crate) const NAME: &[&str] = &["AsRef"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, &parse_quote!(::core::convert::AsRef), None, parse_quote! {
            trait AsRef<__T: ?Sized> {
                #[inline]
                fn as_ref(&self) -> &__T;
            }
        }))
    }
}

pub(crate) mod as_mut {
    use crate::derive::prelude::*;

    pub(crate) const NAME: &[&str] = &["AsMut"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, &parse_quote!(::core::convert::AsMut), None, parse_quote! {
            trait AsMut<__T: ?Sized> {
                #[inline]
                fn as_mut(&mut self) -> &mut __T;
            }
        }))
    }
}

pub(crate) mod into {
    use syn::{spanned::Spanned as _, Error, PathArguments};

    use crate::derive::prelude::*;

    pub(crate) const NAME: &[&str] = &["Into"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        let Some(path) = cx.trait_path() else { unreachable!() };
        let Some(trait_name) = path.segments.last() else { unreachable!() };

        let PathArguments::AngleBracketed(ref into_type) = trait_name.arguments else {
            return Err(Error::new(
                path.span(),
                "Into trait requires a generic argument, eg: Into<TargetType>.",
            ));
        };
        if into_type.args.len() != 1 {
            return Err(Error::new(into_type.span(), "Into trait must take one argument."));
        }
        let target = into_type.args.first().unwrap().clone();
        let path = path.clone();

        let mut enum_impl = derive_utils::EnumImpl::new(data);
        enum_impl.set_trait(path.clone());
        for v in &data.variants {
            let variant_name = &v.ident;
            enum_impl.push_where_predicate(parse_quote! {
                #variant_name: #path
            });
        }
        enum_impl.push_method(parse_quote! {
            #[inline]
            fn into(self) -> #target;
        });
        Ok(enum_impl.build())
    }
}
