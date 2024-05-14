use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub bump: u8,    //1
    pub owner: Pubkey, //32
    pub spot_numbers: Vec<u32>,
    pub game_id: u64,
}


impl User {
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