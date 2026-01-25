use bevy::ecs::resource::Resource;
use rand::{Rng, SeedableRng, distributions::uniform::*, distributions::Standard};
use std::sync::Mutex;

#[cfg(all(not(feature = "xorshift"), not(feature = "pcg")))]
type RngCore = rand::prelude::StdRng;

#[cfg(feature = "xorshift")]
type RngCore = rand_xorshift::XorShiftRng;

#[cfg(feature = "pcg")]
type RngCore = rand_pcg::Pcg64Mcg;

#[derive(Resource)]
pub struct RandomNumberGenerator {
    rng: Mutex<RngCore>
}

impl Default for RandomNumberGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/*
    Supports unsigned 32-bit integer ranges of unsigned 64-bit seeds.
*/
impl RandomNumberGenerator {
    pub fn new() -> Self {
        Self {
            rng: Mutex::new(RngCore::from_entropy()),
        }
    }
    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: Mutex::new(RngCore::seed_from_u64(seed)),
        }
    }
    pub fn range<T>(&mut self, range: impl SampleRange<T>) -> T
    where
        T: SampleUniform + PartialOrd,
    {
        let mut lock = self.rng.lock().unwrap();
        lock.gen_range(range)
    }

    pub fn generic<T>(&mut self) -> T
    where
        // full path declared here for explicitness
        Standard: rand::prelude::Distribution<T>,
    {
        let mut lock = self.rng.lock().unwrap();
        // r# required to use gen as an identifier
        lock.r#gen()
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

pub struct ToyPrngPlugin;

impl bevy::prelude::Plugin for ToyPrngPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(RandomNumberGenerator::new());
    }
}