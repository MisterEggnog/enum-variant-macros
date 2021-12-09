use std::error::Error;
use std::fmt;

/// Errors that can stop the operation.
#[derive(Debug)]
pub struct VariantCastError {
    exp_type: String,
    act_type: String,
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
