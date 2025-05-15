use anchor_lang::prelude::*;

#[error_code]
pub enum TokenExchangeError {
    #[msg("Invalid fee rate configuration")]
    InvalidFeeRate,

    #[msg("Insufficient input amount")]
    InsufficientAmount,

    #[msg("Insufficient liquidity in pool")]
    InsufficientLiquidity,

    #[msg("Calculation overflow occurred")]
    CalculationOverflow,

    #[msg("Invalid token account owner")]
    InvalidOwner,

    #[msg("Operation exceeds maximum allowed slippage")]
    SlippageExceeded,

    #[msg("Pool is in cooldown period")]
    PoolInCooldown,

    #[msg("Token burn failed")]
    BurnFailed,

    #[msg("Unauthorized operation")]
    Unauthorized,
}