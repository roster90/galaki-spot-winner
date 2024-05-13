use anchor_lang::prelude::*;


#[account]
pub struct Galaki {
    pub bump: u8,    //1
    pub owner: Pubkey, //32
    pub count_project: u64,
    pub admin_role: Vec<Pubkey> ,
    pub version: String,
    pub pause: bool,
    pub operator_wallet: Pubkey,
}

impl Galaki {
    pub fn initialize(
        &mut self,
        owner: &Pubkey,
        bump: u8,
        admin_role: &Pubkey,
        operator_wallet: &Pubkey,

    ) -> Result<()> {
        self.owner = *owner;
        self.bump = bump;
        self.count_project = 0;
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
    
    pub fn set_count_project(&mut self, count_project: u64) {
        self.count_project = count_project;
    }
    
    pub fn has_admin(&self, authority: &Pubkey) -> bool {
        self.admin_role.contains(authority)
    }
    
}