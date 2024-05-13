use anchor_lang::prelude::*;




#[account]
pub struct GameProject {
    pub bump: u8,
    pub status: bool,
    pub game_id: u64,
    pub spot_numbers: Vec<u32>,
    pub currency: Pubkey,         
    pub spot_winner: u32,
    pub start_time: i64,
    pub end_time: i64,

}
