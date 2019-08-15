## [`rayon::ParallelExtend`](https://docs.rs/rayon/1.0/rayon/iter/trait.ParallelExtend.html)

When deriving for enum like the following:

```rust
#[enum_derive(rayon::ParallelExtend)]
enum Enum<A, B> {
    A(A),
    B(B),
}
```

Code like this will be generated:

```rust
enum Enum<A, B> {
    A(A),
    B(B),
}

impl<A, B, __T: Send> ::rayon::iter::ParallelExtend<__T> for Enum<A, B>
where
    A: ::rayon::iter::ParallelExtend<__T>,
    B: ::rayon::iter::ParallelExtend<__T>,
{
    #[inline]
    fn par_extend<__I>(&mut self, par_iter: __I)
    where
        __I: ::rayon::IntoParallelIterator<Item = __T>
    {
        match self {
            Enum::A(x) => ::rayon::iter::ParallelExtend::par_extend(x, par_iter),
            Enum::B(x) => ::rayon::iter::ParallelExtend::par_extend(x, par_iter),
        }
    }
}
```
