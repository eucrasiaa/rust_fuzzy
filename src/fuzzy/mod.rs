//! # Fuzzy Algorithms, Session Engine
//!
//! The `fuzzy` module is the heart & soul of the library's search capabilities, 
//! - algoritms!
//! - matching logic!
//! - session state stuff!
//!
//! - **`FuzzyCandidate`**: if implemented, allows for passing to a session!
//! - **`SearchSession`**: state controller(?) manager(?) the thing handling the algorithm.
//!     - history and current query management
//!     - letter by letter parsing
//!     - flat string querying
//!     - results polling
//!     - interfaces for the tui or other handlers if designed!
//!     - threshold for multi character type culling for optimization
pub mod algorithm;
pub mod session;
pub mod canidate;
pub mod matcher;
pub mod algorithms;
// pub mod prelude {
// pub use self::algorithm::*;
pub use self::canidate::*;
pub use self::session::*;
pub use self::matcher::*;
pub use self::algorithms::SimilarityAlgorithm;
pub use self::algorithms::*;
pub use self::canidate::ScoreTarget;

/// the way weights are handled, to optimize calcs, is that its ints scaled as a 1024 and bit
/// shifted. so be sure to use handlers! 
#[inline(always)]
pub fn scale_weight(f: f64) -> i64 {
    (f * 1024.0).round() as i64
}
// pub use self::algorithm::*;
    // Add other common traits/structs here
// }
// pub use self::SimilarityAlgorithm;

