
use anchor_lang::prelude::*;

use crate::{ AuthorityRole, Galaki};

#[derive(Accounts)]
pub struct InitializeGanaki<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 32 + 32 + 1,
        seeds = [],
        bump,

    )]
    pub galaki_account: Account<'info, Galaki>,
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 32 + 32 + 1,
        seeds = [],
        bump,
    )]
    pub owner_account: Account<'info, AuthorityRole>,
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 32 + 32 + 1,
        seeds = [],
        bump,
    )]
    pub admin_account: Account<'info, AuthorityRole>,



    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}