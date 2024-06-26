use anchor_lang::prelude::*;

use rand::Rng;


#[account]
pub struct GameProject {
    pub bump: u8,
    pub status: bool,
    pub id: u64,
    pub spot_numbers: Vec<u32>,
    pub currency: Pubkey,        
    pub price_per_spot: u32, 
    pub spot_winner: u32,
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
    fn get_spot_numbers(&self) -> usize {
        self.spot_numbers.len()
    }

    pub fn handle_set_random_winner( &mut self)-> Result<()> {
        let range = self.get_spot_numbers() as u32;
        let mut rng = rand::thread_rng();
        let n2: u32 = rng.gen_range(0..range);
        msg!("Random number: {}", n2);
        self.spot_winner = n2;
        Ok(())
    }

    pub fn get_winner_number(&mut self) -> u32{
        self.spot_winner
    }

    pub fn is_active(&self) -> bool {
        self.start_time < self.end_time
    }
    
}