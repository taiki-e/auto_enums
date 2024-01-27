use auto_enums::enum_derive;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B> ::core::iter::Iterator for Enum<A, B>
where
    A: ::core::iter::Iterator,
    B: ::core::iter::Iterator<Item = <A as ::core::iter::Iterator>::Item>,
{
    type Item = <A as ::core::iter::Iterator>::Item;
    #[inline]
    fn next(&mut self) -> ::core::option::Option<Self::Item> {
        match self {
            Enum::A(x) => <A as ::core::iter::Iterator>::next(x),
            Enum::B(x) => <B as ::core::iter::Iterator>::next(x),
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, ::core::option::Option<usize>) {
        match self {
            Enum::A(x) => <A as ::core::iter::Iterator>::size_hint(x),
            Enum::B(x) => <B as ::core::iter::Iterator>::size_hint(x),
        }
    }
    #[inline]
    fn count(self) -> usize {
        match self {
            Enum::A(x) => <A as ::core::iter::Iterator>::count(x),
            Enum::B(x) => <B as ::core::iter::Iterator>::count(x),
        }
    }
    #[inline]
    fn last(self) -> ::core::option::Option<Self::Item> {
        match self {
            Enum::A(x) => <A as ::core::iter::Iterator>::last(x),
            Enum::B(x) => <B as ::core::iter::Iterator>::last(x),
        }
    }
    #[inline]
    #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
    fn collect<__U: ::core::iter::FromIterator<Self::Item>>(self) -> __U {
        match self {
            Enum::A(x) => <A as ::core::iter::Iterator>::collect(x),
            Enum::B(x) => <B as ::core::iter::Iterator>::collect(x),
        }
    }
    #[inline]
    fn fold<__U, __F>(self, init: __U, f: __F) -> __U
    where
        __F: ::core::ops::FnMut(__U, Self::Item) -> __U,
    {
        match self {
            Enum::A(x) => <A as ::core::iter::Iterator>::fold(x, init, f),
            Enum::B(x) => <B as ::core::iter::Iterator>::fold(x, init, f),
        }
    }
    #[inline]
    fn find<__P>(&mut self, predicate: __P) -> ::core::option::Option<Self::Item>
    where
        __P: ::core::ops::FnMut(&Self::Item) -> bool,
    {
        match self {
            Enum::A(x) => <A as ::core::iter::Iterator>::find(x, predicate),
            Enum::B(x) => <B as ::core::iter::Iterator>::find(x, predicate),
        }
    }
    #[inline]
    fn find_map<__U, __F>(&mut self, f: __F) -> ::core::option::Option<__U>
    where
        __F: ::core::ops::FnMut(Self::Item) -> ::core::option::Option<__U>,
    {
        match self {
            Enum::A(x) => <A as ::core::iter::Iterator>::find_map(x, f),
            Enum::B(x) => <B as ::core::iter::Iterator>::find_map(x, f),
        }
    }
}
impl<A, B> ::core::iter::FusedIterator for Enum<A, B>
where
    A: ::core::iter::FusedIterator,
    B: ::core::iter::FusedIterator<Item = <A as ::core::iter::Iterator>::Item>,
{}
fn main() {}
