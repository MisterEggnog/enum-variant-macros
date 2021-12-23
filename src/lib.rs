//! This macro library is currently incomplete, it works for enums in a very
//! specific format but it is currently very touchy.
//! Use at your own risk.
pub use try_from_derive_proc::TryFromVariants;

mod variant_cast_error;

pub use variant_cast_error::*;
