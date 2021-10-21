use anyhow::Result;
use rand_core::{impls, Error, RngCore, SeedableRng};
use std::fmt;
use std::num::Wrapping;

#[derive(Clone)]
pub struct SplitMix64(u64);

// Custom Debug implementation that does not expose the internal state
impl fmt::Debug for SplitMix64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SplitMix64 {{}}")
    }
}

impl SplitMix64 {
    /// Creates a new SplitMix64 instance from the given seed.
    pub const fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// Creates a new SplitMix64 instance which is not seeded.
    ///
    /// The initial value of this RNG is a constant, so all generators created
    /// by this function will yield the same stream of random numbers. It is
    /// highly recommended that this is created through `SeedableRng` instead of
    /// this function
    pub const fn new_unseeded() -> Self {
        Self(0x193a6754)
    }

    /// Global random number generator with a unique state value.
    ///
    /// This is safe on 64-bits architectures, where u64 reads and writes
    /// are atomic.
    #[allow(clippy::mut_from_ref)]
    pub fn as_mut(&self) -> &mut Self {
        unsafe {
            let const_ptr = self as *const Self;
            let mut_ptr = const_ptr as *mut Self;
            &mut *mut_ptr
        }
    }
}

impl RngCore for SplitMix64 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let mut z = Wrapping(self.0) + Wrapping(0x9E3779B97F4A7C15_u64);
        self.0 = z.0;
        z = (z ^ (z >> 30)) * Wrapping(0xBF58476D1CE4E5B9_u64);
        z = (z ^ (z >> 27)) * Wrapping(0x94D049BB133111EB_u64);
        (z ^ (z >> 31)).0
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl SeedableRng for SplitMix64 {
    type Seed = [u8; 8];

    fn from_seed(seed: Self::Seed) -> Self {
        Self(u64::from_ne_bytes(seed))
    }

    fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
        let mut seed = [0u8; 8];
        loop {
            rng.try_fill_bytes(&mut seed)?;
            if !seed.iter().all(|&x| x == 0) {
                break;
            }
        }

        Ok(Self::from_seed(seed))
    }
}

impl Default for SplitMix64 {
    fn default() -> Self {
        Self::new_unseeded()
    }
}
