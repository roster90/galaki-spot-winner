use anchor_lang::prelude::*;
use rand::Rng;

declare_id!("GiRmAdyLyhhfseRxeNkbnwRCr1EMqb8nzXEJ2cbP83MU");

pub mod types;
pub mod state;
pub mod error;
pub mod constants;
pub mod events;
pub mod instructions;


pub use constants::*;
pub use error::*;
pub use events::*;
pub use state::*;
pub use types::*;
pub use instructions::*;


#[program]
pub mod galaki_winner_spot_sc {
    use super::*;


    pub fn initialize(ctx: Context<InitializeGanaki>) -> Result<()> {
        

        Ok(())
    }

    pub fn create_project(ctx: Context<CreateGameProject>) -> Result<()> {
        Ok(())
    }
    pub fn user_join_project(ctx: Context<Initialize>) -> Result<()> {
        let mut rng = rand::thread_rng();
        let n2: u32 = rng.gen_range(0..10);
        msg!("Random number: {}", n2);
      
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
