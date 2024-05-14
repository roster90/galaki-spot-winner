use anchor_lang::prelude::*;
use rand::Rng;

declare_id!("GiRmAdyLyhhfseRxeNkbnwRCr1EMqb8nzXEJ2cbP83MU");

pub mod types;
pub mod state;
pub mod error;
pub mod constants;
pub mod events;
pub mod instructions;
pub mod utils;


pub use constants::*;
pub use error::*;
pub use events::*;
pub use state::*;
pub use types::*;
pub use instructions::*;
pub use utils::*;


#[program]
pub mod galaki_winner_spot_sc {
    use super::*;


    //owner fn
    pub fn initialize(ctx: Context<InitializeGanaki>, operator_wallet: Pubkey) -> Result<()> {
        initialize_galaki::handle_initialize_galaki(ctx, operator_wallet)
    }
    //admin fn
    pub fn set_admin(ctx: Context<SetAdmin>, new_admin: Pubkey) -> Result<()> {
        set_auth_role::handle_set_admin(ctx, new_admin)
    }
    //admin fn
    pub fn remove_admin(ctx: Context<RemoveAdmin>, admin: Pubkey) -> Result<()> {
        set_auth_role::handle_remove_admin(ctx, admin)
    }
    //admin fn
    pub fn set_operator(ctx: Context<SetOperator>, new_operator: Pubkey) -> Result<()> {
        set_auth_role::handle_set_operator(ctx, new_operator)
    }

    //admin fn
    pub fn remove_operator(ctx: Context<RemoveOperator>, operator: Pubkey) -> Result<()> {
        set_auth_role::handle_remove_operator(ctx, operator)
    }

    //operator fn
    pub fn create_project(ctx: Context<CreateGameProject>, params: GameInitParams) -> Result<()> {
       create_game_project::handle_create_game_project(ctx, params)
    }

    pub fn player_deposit_game(ctx: Context<Initialize>) -> Result<()> {
        let mut rng = rand::thread_rng();
        let n2: u32 = rng.gen_range(0..10);
        msg!("Random number: {}", n2);
      
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
