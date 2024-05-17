

use anchor_lang::prelude::*;

use crate::GalaKiErrors;





#[account]
pub struct GameProject {
    pub is_close: bool,
    pub bump: u8,
    pub id: u64,
    pub spot_numbers: Vec<u64>,
    pub currency: Pubkey,        
    pub price_per_spot: u32, 
    pub spot_winner: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub total_amount: u64,
    pub rate: u16, //4 decimals
    pub is_user_claim_reward: bool,


}

impl  GameProject {
    pub fn initialize(
        &mut self,
        id: u64,
        currency: Pubkey,
        price_per_spot: u32,
        start_time: i64,
        duration: i64,
        bump: u8,
    ) -> Result<()> {
        self.id = id;
        self.currency = currency;
        self.start_time = start_time;
        self.end_time = start_time + duration;
        self.bump = bump;
        self.is_close = false;
        self.price_per_spot = price_per_spot;
        self.is_user_claim_reward = false;
        Ok(())
    }
    pub fn get_spot_numbers(&self) -> usize {
        self.spot_numbers.len()
    }


    pub fn get_winner_number(&mut self) -> u64{
        self.spot_winner
    }

    pub fn is_active(&self) -> bool {
        self.start_time < self.end_time
    }

    pub fn check_spot(&self, spot: u64) -> bool {
        self.spot_numbers.contains(&spot)
    }

    pub fn set_winner(&mut self, spot: u64) {
        self.spot_winner = spot;
        self.is_close = true;
    }

    fn add_sport_number(&mut self, spot: u64) -> Result<()> {
        if self.spot_numbers.contains(&spot) {
            return Err(GalaKiErrors::SpotAlreadyExist.into());
        }
        self.spot_numbers.push(spot);
        Ok(())
    }

    pub fn get_total_amount(&self) -> u64 {
        self.total_amount
    }
    
    pub fn user_participated_amount(&mut self, spot: u64, participate_amount: u64) -> Result<()>{
        self.add_sport_number(spot)?;
        self.total_amount += participate_amount;
        Ok(())
    }

    pub fn set_user_claim_reward(&mut self, status: bool) -> Result<()>{
        self.is_user_claim_reward = status;
        Ok(())
    }
    
}