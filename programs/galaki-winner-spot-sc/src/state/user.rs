use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

#[account]
pub struct Player {
    pub bump: u8,    //1
    pub owner: Pubkey, //32
    pub spot_numbers: Vec<u64>, //max = 10
    pub is_claimed_reward: bool,
    pub amount_claimed: u64
}


impl Player {
    pub fn initialize(
        &mut self,
        owner: &Pubkey,
        bump: u8,

    ) -> Result<()> {
        self.owner = *owner;
        self.bump = bump;
        Ok(())
    }
    
    pub fn add_spot_number(&mut self, spot_number: u64) {
        //check if spot_number is already in the list
        if self.spot_numbers.contains(&spot_number) {
            return;
        }
        self.spot_numbers.push(spot_number);
    }
    
    pub fn get_spot_numbers(&self) -> Vec<u64> {
        self.spot_numbers.clone()
    }

    pub fn claim_reward(&mut self, amount: u64)-> Result<()> {
        self.is_claimed_reward = true;
        self.amount_claimed = amount;
        Ok(())
    }
    
    
    pub fn get_owner(&self) -> Pubkey {
        self.owner
    }

    pub fn is_winner(&self, spot_number: u64) -> bool {
        self.spot_numbers.contains(&spot_number)
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