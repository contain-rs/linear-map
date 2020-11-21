//! A map implemented by searching linearly in a vector.
//!
//! See the [`LinearMap`](struct.LinearMap.html) type for details.

#![deny(missing_docs)]

mod map;
pub use map::*;

// Optional Serde support
#[cfg(feature = "serde_impl")]
pub mod serde;
pub mod set;
