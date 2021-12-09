use std::error::Error;
use std::fmt;

/// Different variant than expected during TryFrom
#[derive(Debug)]
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
