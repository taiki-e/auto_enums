### Original code:

```rust
#[auto_enum(Iterator)]
fn foo(x: i32) -> impl Iterator<Item = i32> {
    if x < 0 {
        return marker!(x..=0);
    }
    match x {
        0 => 1..10,
        _ => vec![5, 10].into_iter(),
    }
}
```

### Generated code:

```rust
fn foo(x: i32) -> impl Iterator<Item = i32> {
    enum __Enum1<__T1, __T2, __T3> {
        __T1(__T1),
        __T2(__T2),
        __T3(__T3),
    }

    impl<__T1, __T2, __T3> ::std::iter::Iterator for __Enum1<__T1, __T2, __T3>
    where
        __T1: ::std::iter::Iterator,
        __T2: ::std::iter::Iterator<Item = <__T1 as ::std::iter::Iterator>::Item>,
        __T3: ::std::iter::Iterator<Item = <__T1 as ::std::iter::Iterator>::Item>,
    {
        type Item = <__T1 as ::std::iter::Iterator>::Item;
        #[inline]
        fn next(&mut self) -> ::std::option::Option<Self::Item> {
            match self {
                __Enum1::__T1(x) => x.next(),
                __Enum1::__T2(x) => x.next(),
                __Enum1::__T3(x) => x.next(),
            }
        }
        #[inline]
        fn size_hint(&self) -> (usize, ::std::option::Option<usize>) {
            match self {
                __Enum1::__T1(x) => x.size_hint(),
                __Enum1::__T2(x) => x.size_hint(),
                __Enum1::__T3(x) => x.size_hint(),
            }
        }
    }

    if x < 0 {
        return __Enum1::__T3(x..=0);
    }
    match x {
        0 => __Enum1::__T1(1..10),
        _ => __Enum1::__T2(vec![5, 10].into_iter()),
    }
}
```
