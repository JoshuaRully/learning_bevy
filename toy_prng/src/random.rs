use bevy::ecs::resource::Resource;
use rand::{Rng, SeedableRng, distributions::uniform::SampleRange};

#[cfg(all(not(feature = "xorshift"), not(feature = "pcg")))]
type RngCore = rand::prelude::StdRng;

#[cfg(feature = "xorshift")]
type RngCore = rand_xorshift::XorShiftRng;

#[cfg(feature = "pcg")]
type RngCore = rand_pcg::Pcg64Mcg;

/// `RandomNumberGenerator` holds random number generation state, and offers
/// random number generation services to your program.
///
/// `RandomNumberGenerator` defaults to using the
/// [PCG](https://crates.io/crates/rand_pcg) algorithm.
/// You can specify `xorshift` as a feature flag to use it instead.
/// By default, `RandomNumberGenerator` requires mutability---it
/// is shared in Bevy with `ResMut<RandomNumberGenerator>`. If
/// you prefer interior mutability (and wish to use
/// `Res<RandomNumberGenerator>` instead), specify the `locking` feature flag.
///
/// ## Example
///
/// ```
/// use my_library::RandomNumberGenerator;
/// let mut my_rng = RandomNumberGenerator::new();
/// let random_number = my_rng.range(1..10);
/// println!("{random_number}");
/// ```
#[derive(Resource)]
pub struct RandomNumberGenerator {
    rng: RngCore
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
    /// Creates a default `RandomNumberGenerator`, with a randomly selected starting seed.
    pub fn new() -> Self {
        Self {
            rng: RngCore::from_entropy(),
        }
    }
    /// Creates a new `RandomNumberGenerator`, with a user-specified random seed.
    /// It will produce the same results each time (given the same requests).
    ///
    /// # Arguments ///
    /// * `seed` - the random seed to use.
    ///
    /// # Example
    ///
    /// ```
    /// use my_library::RandomNumberGenerator;
    /// let mut rng1 = RandomNumberGenerator::seeded(1);
    /// let mut rng2 = RandomNumberGenerator::seeded(1);
    /// let results: (u32, u32) = ( rng1.next(), rng2.next() ); /// assert_eq!(results.0, results.1);
    /// ```
    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: RngCore::seed_from_u64(seed),
        }
    }
    /// Generates a random number within the specified range.
    ///
    /// # Arguments
    ///
    /// * `range` - the range (inclusive or exclusive) within which to /// generate a random number.
    ///
    /// # Example
    ///
    /// ```
    /// use my_library::RandomNumberGenerator;
    /// let mut rng = RandomNumberGenerator::new();
    /// let one_to_nine = rng.range(1..10);
    /// let one_to_ten = rng.range(1..=10);
    /// ```
    pub fn range<T>(&mut self, range: impl SampleRange<T>) -> T
    where
        T: rand::distributions::uniform::SampleUniform + PartialOrd,
    {
        self.rng.gen_range(range)
    }
    /// Generates a new random number of the requested type.
    pub fn generic<T>(&mut self) -> T
    where
        // full path declared here for explicitness
        rand::distributions::Standard: rand::prelude::Distribution<T>,
    {
        // r# required to use gen as an identifier
        self.rng.r#gen()
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

/// `Random` is a Bevy plugin that inserts a `RandomNumberGenerator`
/// resource into your application.
///
/// Once you add the plugin (with `App::new().add_plugin(Random)`),
/// you can access a random number generator in systems with
/// `rng: ResMut<RandomNumberGenerator>`.
pub struct ToyPrngPlugin;

impl bevy::prelude::Plugin for ToyPrngPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(RandomNumberGenerator::new());
    }
}