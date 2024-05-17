

use anchor_lang::prelude::*;

use crate::GalaKiErrors;





#[account]
pub struct GameProject {
    pub bump: u8,
    pub status: bool,
    pub id: u64,
    pub spot_numbers: Vec<u64>,
    pub currency: Pubkey,        
    pub price_per_spot: u32, 
    pub spot_winner: u64,
    pub start_time: i64,
    pub end_time: i64,

}

impl  GameProject {
    pub fn initialize(
        &mut self,
        id: u64,
        currency: Pubkey,
        price_per_spot: u32,
        start_time: i64,
        end_time: i64,
        bump: u8,
  
    ) -> Result<()> {
        self.id = id;
        self.currency = currency;
        self.start_time = start_time;
        self.end_time = end_time;
        self.bump = bump;
        self.status = true;
        self.price_per_spot = price_per_spot;
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
    }

    pub fn add_sport_number(&mut self, spot: u64) -> Result<()> {
        
        if self.spot_numbers.contains(&spot) {
            return Err(GalaKiErrors::SpotAlreadyExist.into());
        }
        self.spot_numbers.push(spot);
        Ok(())
    }
    
}