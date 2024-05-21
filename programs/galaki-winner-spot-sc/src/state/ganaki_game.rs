use anchor_lang::prelude::*;

use crate::GalaKiErrors;

#[account]
pub struct GalakiGame {
    pub bump: u8,           //1
    pub version: String,    //8
    pub pause: bool,        //1
    pub admin: Vec<Pubkey>, //max 3 : 8+ 32*3 = 104

    pub is_close: bool,           //1
    pub spot_numbers: Vec<u64>,   // 8 + 8*max_ticket = 8 + 8*1000 = 808
    pub currency: Pubkey,         //32
    pub price_ticket: u32,        //4
    pub spot_winner: u64,         //8
    pub open_timestamp: i64,      //8
    pub end_timestamp: i64,       //8
    pub total_ticket: u16,        //2
    pub max_ticket_per_user: u16, //2
    pub max_ticket: u16,          //2

    //for reward
    pub is_reward_active: bool,  //1
    pub reward_currency: Pubkey, //32
    pub reward_amount: u64,      //8
}

impl GalakiGame {
    pub fn initialize(
        &mut self,
        currency: Pubkey,
        price_ticket: u32,
        max_ticket: u16,
        max_ticket_per_user: u16,
        start_time: i64,
        duration: i64,
        admin: Pubkey,
        bump: u8,
    ) -> Result<()> {
        self.bump = bump;
        self.version = String::from("1.0");
        self.pause = false;
        self.admin.push(admin);
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

    pub fn set_pause(&mut self, pause: bool) {
        self.pause = pause;
    }

    pub fn has_admin(&self, authority: Pubkey) -> bool {
        self.admin.contains(&authority)
    }

    pub fn set_admin(&mut self, authority: Pubkey) -> Result<()> {
        self.admin.push(authority);
        Ok(())
    }
    pub fn remove_admin(&mut self, authority: Pubkey) -> Result<()> {
        self.admin.retain(|&x| x != authority);
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

        if self.end_timestamp < current_time
            || self.is_close
            || self.total_ticket >= self.max_ticket
        {
            return 1; //close
        }

        if !self.is_reward_active {
            return 3; //reward is not active
        }

        return 4; //other
    }

    pub fn get_spot_numbers(&self) -> usize {
        self.spot_numbers.len()
    }

    pub fn get_winner_number(&mut self) -> u64 {
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

    pub fn user_participated_amount(&mut self, spot: u64) -> Result<()>{
        self.add_sport_number(spot)?;
        self.total_ticket += 1;
        Ok(())
    }
}
