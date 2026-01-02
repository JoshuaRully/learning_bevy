use rand::{Rng, SeedableRng, distributions::uniform::SampleRange, prelude::StdRng};

pub struct RandomNumberGenerator {
    rng: StdRng,
}

/*
    Supports unsigned 32-bit integer ranges of unsigned 64-bit seeds.
*/
impl RandomNumberGenerator {
    pub fn new() -> Self {
        Self {
            rng: StdRng::from_entropy(),
        }
    }
    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }
    pub fn range<T>(&mut self, range: impl SampleRange<T>) -> T
    where
        T: rand::distributions::uniform::SampleUniform + PartialOrd,
    {
        self.rng.gen_range(range)
    }

    pub fn generic<T>(&mut self) -> T
    where
        // full path declared here for explicitness
        rand::distributions::Standard: rand::prelude::Distribution<T>,
    {
        // r# required to use gen as an identifier
        self.rng.r#gen()
    }
}

impl Default for RandomNumberGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_range_bounds() {
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..1000 {
            let num = rng.range(1..10);
            assert!(num >= 1);
            assert!(num < 10);
        }
    }
    #[test]
    fn test_determinism() {
        let mut rng = (
            RandomNumberGenerator::seeded(1),
            RandomNumberGenerator::seeded(1),
        );
        (0..1000).for_each(|_| {
            assert_eq!(
                rng.0.range(u32::MIN..u32::MAX),
                rng.1.range(u32::MIN..u32::MAX),
            )
        });
    }
    #[test]
    fn test_generic_types() {
        let mut rng = RandomNumberGenerator::new();
        let _: i32 = rng.generic();
        let _ = rng.generic::<f32>();
    }
    #[test]
    fn test_float() {
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..1000 {
            let n = rng.range(-5000.0f32..5000.0f32);
            assert!(n.is_finite());
            assert!(n > -5000.0);
            assert!(n < 5000.0);
        }
    }
}
