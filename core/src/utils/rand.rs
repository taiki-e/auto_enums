use rand_core::{RngCore, SeedableRng};

// =============================================================================
// XorShiftRng

pub(crate) use rand_xorshift::XorShiftRng;

pub(crate) fn xorshift_rng() -> XorShiftRng {
    const SEED: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    XorShiftRng::from_seed(SEED)
}

// =============================================================================
// Rng

pub(crate) trait Rng: RngCore {
    #[inline]
    fn gen<T>(&mut self) -> T
    where
        Standard: Distribution<T>,
    {
        Standard.sample(self)
    }
}

impl<R: RngCore + ?Sized> Rng for R {}

// =============================================================================
// Distribution

pub(crate) trait Distribution<T> {
    /// Generate a random value of `T`, using `rng` as the source of randomness.
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T;
}

impl<T, D: Distribution<T>> Distribution<T> for &D {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {
        (*self).sample(rng)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Standard;

impl Distribution<u32> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u32 {
        rng.next_u32()
    }
}
