//! This macro library is currently incomplete, it works for enums in a very
//! specific format but it is has some rough spots.
//!
//! To use the TryFromVariants macro, the type needs to provide a `From<YourEnum>` to `&'static str` for for this derivation to
//! succeed.
//! I recommend using [strum_macros::IntoStaticStr](https://docs.rs/strum/0.23.0/strum/derive.IntoStaticStr.html).
//! ## Warning
//! Note that this only works for enums composed solely of 1 member unnamed variant.
//!
//! ## Example
//! ```
//! use enum_variant_macros::*;
//! use strum_macros::IntoStaticStr;
//!
//! #[derive(PartialEq, Debug, IntoStaticStr, FromVariants, TryFromVariants)]
//! enum Wrap {
//!     Float(f32),
//!     Int(i32),
//! }
//! assert_eq!(Wrap::Int(4), Wrap::from(4_i32));
//! assert_eq!(Ok(4_i32), i32::try_from(Wrap::Int(4)));
//! assert!(f32::try_from(Wrap::Int(4)).is_err());
//! ```
use std::error::Error;
use std::fmt;

pub use enum_variant_macros_macros::FromVariants;
pub use enum_variant_macros_macros::TryFromVariants;

/// Different variant than expected during TryFrom
#[derive(Debug, PartialEq, Eq)]
pub struct VariantCastError {
    /// Enum type
    pub enum_type: &'static str,
    /// Expected variant
    pub exp_type: &'static str,
    /// Actual variant
    pub variant_name: &'static str,
}

impl fmt::Display for VariantCastError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Casting {} to {} failed, variant was {}.",
            self.enum_type, self.exp_type, self.variant_name
        )
    }
}

impl Error for VariantCastError {}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_display_string() {
        let error = VariantCastError {
            enum_type: "Carl",
            exp_type: "Fish",
            variant_name: "Bread",
        };
        let displayed = format!("{}", error);
        assert_eq!("Casting Carl to Fish failed, variant was Bread.", displayed);
    }
}
