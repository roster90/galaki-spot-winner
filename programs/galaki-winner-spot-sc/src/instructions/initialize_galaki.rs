
use anchor_lang::prelude::*;

use crate::{ AuthorityRole, Galaki, AuthRole};
use crate::{ ADMIN_ROLE, GALAKI_WINNER};


#[derive(Accounts)]
pub struct InitializeGanaki<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 32 + 32 + 1,
        seeds = [GALAKI_WINNER],
        bump,
    )]
    pub galaki_account: Account<'info, Galaki>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 32 + 32 + 1,
        seeds = [ADMIN_ROLE, authority.key().as_ref() ],
        bump,
    )]
    pub admin_account: Account<'info, AuthorityRole>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_galaki(ctx: Context<InitializeGanaki>,operater_wallet: Pubkey ) -> Result<()> {
    let galaki = &mut ctx.accounts.galaki_account;
    let admin = &mut ctx.accounts.admin_account;
    let authority = &ctx.accounts.authority;
    galaki.initialize(&authority.key(), &operater_wallet, ctx.bumps.galaki_account)?;
    admin.initialize(&authority.key(), ctx.bumps.admin_account, AuthRole::Admin)?;

    Ok(())
}