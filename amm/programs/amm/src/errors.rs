use anchor_lang::{error::Error, error_code};
use constant_product_curve::CurveError;

#[error_code]
pub enum AmmError {
    #[msg("Invalid Fee")]
    InvalidFee,
    #[msg("Invalid Auth Bump")]
    AuthBumpError,
    #[msg("Invalid Config Bump")]
    ConfigBumpError,
    #[msg("Invalid Mint Bump")]
    MintLPBumpError,
    #[msg("Invalid Authority")]
    InvalidAuthority,
    #[msg("This pool is locked")]
    Locked,
    #[msg("Expired")]
    Expired,
}

impl From<CurveError> for AmmError {
    fn from(error: CurveError) -> AmmError {
        match error {
            CurveError::InvalidPrecision => AmmError::InvalidPrecision,
            CurveError::Overflow => AmmError::Overflow,
            CurveError::Underflow => AmmError::Underflow,
            CurveError::InvalidFeeAmount => AmmError::InvalidFeeAmount,
            CurveError::InsufficientBalance => AmmError::InsufficientBalance,
            CurveError::InvalidPrecision => AmmError::InvalidPrecision,
            CurveError::ZeroBalance => AmmError::ZeroBalance,
            CurveError::SlippageLimitExceeded => AmmError::SlippageLimitExceeded,
        }
    }
}
