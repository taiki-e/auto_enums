use crate::utils::*;

pub(crate) const NAME: &[&str] = &["rayon::ParallelExtend"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    let iter = quote!(::rayon::iter);

    derive_trait!(
        data,
        parse_quote!(#iter::ParallelExtend)?,
        parse_quote! {
            trait ParallelExtend<__T: Send> {
                fn par_extend<__I>(&mut self, par_iter: __I)
                where
                    __I: #iter::IntoParallelIterator<Item = __T>;
            }
        }?,
    )
    .map(|item| items.push(item))
}
