//! This macro library is currently unstable, it works for it's role in Nostromo
//! Engine, but is not in the state for general use.
//! It may fail in many ways & it currently requires linkage to Nostromo for
//! the error type.
//! Use at your own risk.
//! Actually, your probably better off just not using it at all.
pub use try_from_derive_proc::TryFromVariants;

mod variant_cast_error;

pub use variant_cast_error::*;
