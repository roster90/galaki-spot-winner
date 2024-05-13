use anchor_lang::prelude::*;



#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum AuthRole {
    Admin,
    Operator,
}

#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
pub enum GameStatus {
    Waiting,
    Open,
    Close,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy)]
pub struct GameInitParams {
    pub start_time: i64,     //8
    pub end_time: i64,  //8
    pub currency: Pubkey
}
