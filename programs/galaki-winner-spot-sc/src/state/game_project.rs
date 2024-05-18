

use anchor_lang::prelude::*;

use crate::GalaKiErrors;





#[account]
pub struct GameProject {
    pub is_close: bool,
    pub bump: u8,
    pub id: u64,
    pub spot_numbers: Vec<u64>,
    pub currency: Pubkey,        
    pub price_ticket: u32, 
    pub spot_winner: u64,
    pub open_timestamp: i64,
    pub end_timestamp: i64,
    pub total_ticket: u16,
    pub max_ticket_per_user: u16,
    pub max_ticket: u16,

    //for reward
    pub is_reward_active: bool,
    pub reward_currency: Pubkey,
    pub reward_amount: u64,

}

impl  GameProject {
    pub fn initialize(
        &mut self,
        id: u64,
        currency: Pubkey,
        price_ticket: u32,
        max_ticket: u16,
        max_ticket_per_user: u16,
        start_time: i64,
        duration: i64,
        bump: u8,
    ) -> Result<()> {
        self.id = id;
        self.currency = currency;
        self.open_timestamp = start_time;
        self.end_timestamp = start_time + duration;
        self.bump = bump;
        self.is_close = false;
        self.price_ticket = price_ticket;
        self.max_ticket = max_ticket;
        self.max_ticket_per_user = max_ticket_per_user;
        Ok(())
    }
    pub fn get_spot_numbers(&self) -> usize {
        self.spot_numbers.len()
    }


    pub fn get_winner_number(&mut self) -> u64{
        self.spot_winner
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
    pub fn get_status(&self) -> u8 {

        let current_time = Clock::get().unwrap().unix_timestamp;
            if self.open_timestamp < current_time {
                return 0; //upcoming
            }
            if self.open_timestamp < current_time && self.end_timestamp > current_time {
                return 2; //open
            }

            if self.end_timestamp < current_time  || self.is_close || self.total_ticket >= self.max_ticket{
                return 1; //close
            }



 
            return  3; //other
    }

    // fn add_sport_numbers(&mut self, spot: Vec<u64>) -> Result<()> {
    //     if self.spot_numbers.contains(&spot) {
    //         return Err(GalaKiErrors::SpotAlreadyExist.into());
    //     }
    //     self.spot_numbers.push(spot);
    //     Ok(())
    // }




    pub fn user_participated_amount(&mut self, spot: u64) -> Result<()>{
        self.add_sport_number(spot)?;
        self.total_ticket += 1;
        Ok(())
    }

    
}