
use anchor_lang::prelude::*;

use crate::GameProject;

#[derive(Accounts)]
pub struct CreateGameProject<'info> {

    #[account(
        seeds = [],
        bump,

    )]
pub game_project: Account<'info, GameProject>,



    #[account(mut)]
    pub user: Signer<'info>,
}