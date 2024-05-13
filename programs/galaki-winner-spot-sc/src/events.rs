use anchor_lang::prelude::*;

use crate::{AuthRole};

#[event]
pub struct DepositEvent {
    pub token: Pubkey,
    pub user: Pubkey,
    pub time: i64,
}

#[event]
pub struct SetAuthorityEvent {
    pub admin: Pubkey,
    pub role: AuthRole,
    pub operators: Vec<Pubkey>,
    pub time: i64,
}
