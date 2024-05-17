use anchor_lang::prelude::*;

use crate::AuthRole;

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

#[event]
pub struct ChangeOperatorWalletEvent {
    pub admin: Pubkey,
    pub operator_wallet: Pubkey,
    pub time: i64,
}



#[event]
pub struct CreateGameEvent {
    pub id: u64,
    pub game_account: Pubkey,
    pub currency: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
}

#[event]
pub struct UserParticipateEvent {
    pub game_id: u64,
    pub time: i64,
    pub user: Pubkey,
    pub sport_numbers: u64,
}
#[event]
pub struct RandomnessRequested {
    pub vrf_client: Pubkey,
    pub max_result: u64,
    pub timestamp: i64,
}

