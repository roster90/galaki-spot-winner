use anchor_lang::prelude::*;

#[error_code]
pub enum GalaKiErrors {


    #[msg("Token not support")]
    TokenNotSupport,

    #[msg("Deposit closed")]
    DepositClosed,

    #[msg("Admin account invalid")]
    AdminAccountInvalid,

    #[msg("Operator account invalid")]
    OperatorAccountInvalid,

    #[msg("Only Operator")]
    OnlyOperator,
    #[msg("Only Admin")]
    OnlyAdmin,
    #[msg("Same the wallet address as the new operator wallet address")]
    OperatorWalletSameAsNewWallet,
    #[msg("Time invalid")]
    TimeInvalid,

    #[msg("Game project inactive")]
    GameProjectInactive,

    #[msg(" token account not match")]
    TokenAccountNotMatch,

    #[msg("Insufficient balance")]
    InsufficientBalance,

}

impl From<GalaKiErrors> for ProgramError {
    fn from(e: GalaKiErrors) -> Self {
        ProgramError::Custom(e as u32)
    }
}