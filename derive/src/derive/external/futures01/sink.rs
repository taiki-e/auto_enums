use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures01::Sink"];

pub(crate) fn derive(data: &Data, stack: &mut Stack<ItemImpl>) -> Result<()> {
    let crate_ = quote!(::futures);

    derive_trait!(
        data,
        parse_quote!(#crate_::sink::Sink)?,
        parse_quote! {
            trait Sink {
                type SinkItem;
                type SinkError;
                #[inline]
                fn start_send(&mut self, item: Self::SinkItem) -> #crate_::StartSend<Self::SinkItem, Self::SinkError>;
                #[inline]
                fn poll_complete(&mut self) -> #crate_::Poll<(), Self::SinkError>;
                #[inline]
                fn close(&mut self) -> #crate_::Poll<(), Self::SinkError>;
            }
        }?,
    )
    .map(|item| stack.push(item))
}
