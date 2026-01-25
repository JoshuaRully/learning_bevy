#![warn(missing_docs)]
//!
//!  `toy_prng` provides a suite of helpers to create games with Bevy
//!
//!  ## Overview
//!
//!  * Random number generation facilities.
//!
//!  ## Feature Flags
//!
//!  The following features flags are supported: `xorshift`, `pcg`, `locking`.
//!
//!  ### Random Number Generation
//!
//!  * The `locking` features enables interior mutability inside
//!    [`RandomNumberGenerator`],
//!    allowing it to be used as a resource (`Res<RandomNumberGenerator>`)
//!    rather than requiring mutability (`ResMut<RandomNumberGenerator>`)
//!
//!  * You can control which random number generation algorithm is used by specifying *one* of:
//!    * `xorshift` to use the XorShift algorithm
//!    * `pcg` to use the PCG algorthim
//!
pub use rand;
#[cfg(not(feature = "locking"))]
mod random;
#[cfg(not(feature = "locking"))]
pub use random::*;

#[cfg(feature = "locking")]
mod random_locking;
#[cfg(feature = "locking")]
pub use random_locking::*;