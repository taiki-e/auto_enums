use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
#[allow(unsafe_code)]
impl<A, B, Item> ::futures::sink::Sink<Item> for Enum<A, B>
where
    A: ::futures::sink::Sink<Item>,
    B: ::futures::sink::Sink<Item, Error = <A as ::futures::sink::Sink<Item>>::Error>,
{
    type Error = <A as ::futures::sink::Sink<Item>>::Error;
    #[inline]
    fn poll_ready(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::core::result::Result<(), Self::Error>> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    ::futures::sink::Sink::poll_ready(::core::pin::Pin::new_unchecked(x), cx)
                }
                Enum::B(x) => {
                    ::futures::sink::Sink::poll_ready(::core::pin::Pin::new_unchecked(x), cx)
                }
            }
        }
    }
    #[inline]
    fn start_send(
        self: ::core::pin::Pin<&mut Self>,
        item: Item,
    ) -> ::core::result::Result<(), Self::Error> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    ::futures::sink::Sink::start_send(::core::pin::Pin::new_unchecked(x), item)
                }
                Enum::B(x) => {
                    ::futures::sink::Sink::start_send(::core::pin::Pin::new_unchecked(x), item)
                }
            }
        }
    }
    #[inline]
    fn poll_flush(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::core::result::Result<(), Self::Error>> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    ::futures::sink::Sink::poll_flush(::core::pin::Pin::new_unchecked(x), cx)
                }
                Enum::B(x) => {
                    ::futures::sink::Sink::poll_flush(::core::pin::Pin::new_unchecked(x), cx)
                }
            }
        }
    }
    #[inline]
    fn poll_close(
        self: ::core::pin::Pin<&mut Self>,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<::core::result::Result<(), Self::Error>> {
        unsafe {
            match self.get_unchecked_mut() {
                Enum::A(x) => {
                    ::futures::sink::Sink::poll_close(::core::pin::Pin::new_unchecked(x), cx)
                }
                Enum::B(x) => {
                    ::futures::sink::Sink::poll_close(::core::pin::Pin::new_unchecked(x), cx)
                }
            }
        }
    }
}
fn main() {}
