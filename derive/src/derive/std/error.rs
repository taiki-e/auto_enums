use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Error"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let trait_ = quote!(#root::error::Error);

    #[cfg(all(stable_1_33, not(feature = "error_cause")))]
    let cause = TokenStream::new();
    #[cfg(any(not(stable_1_33), feature = "error_cause"))]
    let cause = quote! {
        #[allow(deprecated)]
        fn cause(&self) -> #root::option::Option<&dyn (#trait_)>;
    };

    derive_trait!(
        data,
        syn::parse2(trait_.clone())?,
        syn::parse2(quote! {
            trait Error {
                fn description(&self) -> &str;
                #cause
                fn source(&self) -> #root::option::Option<&(dyn (#trait_) + 'static)>;
            }
        })?
    )
}
