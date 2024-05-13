use anchor_lang::prelude::*;


#[account]
pub struct Galaki {
    pub bump: u8,    //1
    pub counter: u64,
    pub admin_role: Vec<Pubkey> ,
    pub version: String,
    pub pause: bool,
    pub operator_wallet: Pubkey,
}

impl Galaki {
    pub fn initialize(
        &mut self,
        admin_role: &Pubkey,
        operator_wallet: &Pubkey,
        bump: u8,

    ) -> Result<()> {
        self.bump = bump;
        self.counter = 1;
        self.version = String::from("1.0");
        self.pause = false;
        self.admin_role.push(*admin_role);
        self.operator_wallet = *operator_wallet;
        Ok(())
    }
    
    pub fn set_pause(&mut self, pause: bool) {
        self.pause = pause;
    }
    
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }
    
    pub fn auto_increase_game(&mut self) {
        self.counter = self.counter + 1;
    }
    
    pub fn has_admin(&self, authority: Pubkey) -> bool {
        self.admin_role.contains(&authority)
    }

    pub fn set_admin(&mut self, authority: Pubkey)-> Result<()> {
        self.admin_role.push(authority);
        Ok(())
    }
    pub fn remove_admin(&mut self, authority: Pubkey)-> Result<()> {
        self.admin_role.retain(|&x| x != authority);
        Ok(())
    }
    
}