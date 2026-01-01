use rand::{Rng, SeedableRng, prelude::StdRng};
use std::ops::Range;

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
    pub fn range(&mut self, range: Range<u32>) -> u32 {
        self.rng.gen_range(range)
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
}
