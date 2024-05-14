use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

#[account]
pub struct Player {
    pub bump: u8,    //1
    pub owner: Pubkey, //32
    pub spot_numbers: Vec<u32>,
    pub game_id: u64,
}


impl Player {
    pub fn initialize(
        &mut self,
        owner: &Pubkey,
        bump: u8,
        game_id: u64,
    ) -> Result<()> {
        self.owner = *owner;
        self.bump = bump;
        self.game_id = game_id;
        Ok(())
    }
    
    pub fn add_spot_number(&mut self, spot_number: u32) {
        //check if spot_number is already in the list
        if self.spot_numbers.contains(&spot_number) {
            return;
        }
        self.spot_numbers.push(spot_number);
    }
    
    pub fn get_spot_numbers(&self) -> Vec<u32> {
        self.spot_numbers.clone()
    }
    
    pub fn get_game_id(&self) -> u64 {
        self.game_id
    }
    
    pub fn get_owner(&self) -> Pubkey {
        self.owner
    }
    
}



#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct State {
    pub random_numbers : [f64; 256]
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum RNGMethod {
    Xorshift,
    Hash,
    FastHash,
    None
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct RNGMeta {
    pub initial_seed : u64,
    pub method : RNGMethod
}

pub struct HashStruct {
    pub nonce : u64,
    pub initial_seed : u64
}