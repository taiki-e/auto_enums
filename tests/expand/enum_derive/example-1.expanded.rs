fn foo(x: i32) -> impl Iterator<Item = i32> {
    enum __Enum1<__T1, __T2> {
        __T1(__T1),
        __T2(__T2),
    }
    impl<__T1, __T2> ::core::iter::Iterator for __Enum1<__T1, __T2>
    where
        __T1: ::core::iter::Iterator,
        __T2: ::core::iter::Iterator<Item = <__T1 as ::core::iter::Iterator>::Item>,
    {
        type Item = <__T1 as ::core::iter::Iterator>::Item;
        #[inline]
        fn next(&mut self) -> ::core::option::Option<Self::Item> {
            match self {
                __Enum1::__T1(x) => <__T1 as ::core::iter::Iterator>::next(x),
                __Enum1::__T2(x) => <__T2 as ::core::iter::Iterator>::next(x),
            }
        }
        #[inline]
        fn size_hint(&self) -> (usize, ::core::option::Option<usize>) {
            match self {
                __Enum1::__T1(x) => <__T1 as ::core::iter::Iterator>::size_hint(x),
                __Enum1::__T2(x) => <__T2 as ::core::iter::Iterator>::size_hint(x),
            }
        }
        #[inline]
        fn count(self) -> usize {
            match self {
                __Enum1::__T1(x) => <__T1 as ::core::iter::Iterator>::count(x),
                __Enum1::__T2(x) => <__T2 as ::core::iter::Iterator>::count(x),
            }
        }
        #[inline]
        fn last(self) -> ::core::option::Option<Self::Item> {
            match self {
                __Enum1::__T1(x) => <__T1 as ::core::iter::Iterator>::last(x),
                __Enum1::__T2(x) => <__T2 as ::core::iter::Iterator>::last(x),
            }
        }
        #[inline]
        fn nth(&mut self, n: usize) -> ::core::option::Option<Self::Item> {
            match self {
                __Enum1::__T1(x) => <__T1 as ::core::iter::Iterator>::nth(x, n),
                __Enum1::__T2(x) => <__T2 as ::core::iter::Iterator>::nth(x, n),
            }
        }
        #[inline]
        #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
        fn collect<__U: ::core::iter::FromIterator<Self::Item>>(self) -> __U {
            match self {
                __Enum1::__T1(x) => <__T1 as ::core::iter::Iterator>::collect(x),
                __Enum1::__T2(x) => <__T2 as ::core::iter::Iterator>::collect(x),
            }
        }
        #[inline]
        fn partition<__U, __F>(self, f: __F) -> (__U, __U)
        where
            __U: ::core::default::Default + ::core::iter::Extend<Self::Item>,
            __F: ::core::ops::FnMut(&Self::Item) -> bool,
        {
            match self {
                __Enum1::__T1(x) => <__T1 as ::core::iter::Iterator>::partition(x, f),
                __Enum1::__T2(x) => <__T2 as ::core::iter::Iterator>::partition(x, f),
            }
        }
        #[inline]
        fn fold<__U, __F>(self, init: __U, f: __F) -> __U
        where
            __F: ::core::ops::FnMut(__U, Self::Item) -> __U,
        {
            match self {
                __Enum1::__T1(x) => <__T1 as ::core::iter::Iterator>::fold(x, init, f),
                __Enum1::__T2(x) => <__T2 as ::core::iter::Iterator>::fold(x, init, f),
            }
        }
        #[inline]
        fn all<__F>(&mut self, f: __F) -> bool
        where
            __F: ::core::ops::FnMut(Self::Item) -> bool,
        {
            match self {
                __Enum1::__T1(x) => <__T1 as ::core::iter::Iterator>::all(x, f),
                __Enum1::__T2(x) => <__T2 as ::core::iter::Iterator>::all(x, f),
            }
        }
        #[inline]
        fn any<__F>(&mut self, f: __F) -> bool
        where
            __F: ::core::ops::FnMut(Self::Item) -> bool,
        {
            match self {
                __Enum1::__T1(x) => <__T1 as ::core::iter::Iterator>::any(x, f),
                __Enum1::__T2(x) => <__T2 as ::core::iter::Iterator>::any(x, f),
            }
        }
        #[inline]
        fn find<__P>(&mut self, predicate: __P) -> ::core::option::Option<Self::Item>
        where
            __P: ::core::ops::FnMut(&Self::Item) -> bool,
        {
            match self {
                __Enum1::__T1(x) => <__T1 as ::core::iter::Iterator>::find(x, predicate),
                __Enum1::__T2(x) => <__T2 as ::core::iter::Iterator>::find(x, predicate),
            }
        }
        #[inline]
        fn find_map<__U, __F>(&mut self, f: __F) -> ::core::option::Option<__U>
        where
            __F: ::core::ops::FnMut(Self::Item) -> ::core::option::Option<__U>,
        {
            match self {
                __Enum1::__T1(x) => <__T1 as ::core::iter::Iterator>::find_map(x, f),
                __Enum1::__T2(x) => <__T2 as ::core::iter::Iterator>::find_map(x, f),
            }
        }
        #[inline]
        fn position<__P>(&mut self, predicate: __P) -> ::core::option::Option<usize>
        where
            __P: ::core::ops::FnMut(Self::Item) -> bool,
        {
            match self {
                __Enum1::__T1(x) => {
                    <__T1 as ::core::iter::Iterator>::position(x, predicate)
                }
                __Enum1::__T2(x) => {
                    <__T2 as ::core::iter::Iterator>::position(x, predicate)
                }
            }
        }
    }
    match x {
        0 => __Enum1::__T1(1..10),
        _ => {
            __Enum1::__T2(
                <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([5, 10]))
                    .into_iter(),
            )
        }
    }
}
