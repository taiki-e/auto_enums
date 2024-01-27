extern crate rayon_crate as rayon;
use auto_enums::enum_derive;
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
        __I: ::rayon::iter::IntoParallelIterator<Item = __T>,
    {
        match self {
            Enum::A(x) => {
                <A as ::rayon::iter::ParallelExtend<__T>>::par_extend(x, par_iter)
            }
            Enum::B(x) => {
                <B as ::rayon::iter::ParallelExtend<__T>>::par_extend(x, par_iter)
            }
        }
    }
}
fn main() {}
