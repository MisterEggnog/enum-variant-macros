use std::error::Error;
use std::fmt;

/// Different variant than expected during TryFrom
#[derive(Debug)]
pub struct VariantCastError {
    /// Expected type
    pub exp_type: String,
    /// Actual type
    pub act_type: String,
}

impl fmt::Display for VariantCastError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Casting {} to {} failed, variant was {}",
            "blem", self.exp_type, self.act_type
        )
    }
}

impl Error for VariantCastError {}
