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

}

impl From<GalaKiErrors> for ProgramError {
    fn from(e: GalaKiErrors) -> Self {
        ProgramError::Custom(e as u32)
    }
}