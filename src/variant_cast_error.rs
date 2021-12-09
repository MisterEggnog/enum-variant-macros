use std::error::Error;
use std::fmt;

/// Different variant than expected during TryFrom
#[derive(Debug)]
pub struct VariantCastError {
    /// Enum type
    pub enum_type: String,
    /// Expected variant
    pub exp_type: String,
    /// Actual variant
    pub variant_name: String,
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
            enum_type: String::from("Carl"),
            exp_type: String::from("Fish"),
            variant_name: String::from("Bread"),
        };
        let displayed = format!("{}", error);
        assert_eq!("Casting Carl to Fish failed, variant was Bread.", displayed);
    }
}
