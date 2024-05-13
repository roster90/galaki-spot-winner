use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub bump: u8,    //1
    pub owner: Pubkey, //32
    pub spot_numbers: Vec<u32>,
    pub game_id: u64,
}

