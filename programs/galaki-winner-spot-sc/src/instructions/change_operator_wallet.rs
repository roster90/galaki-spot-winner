use anchor_lang::prelude::*;

use crate::{ AuthorityRole, Galaki, AuthRole, GalaKiErrors, ChangeOperatorWalletEvent};
use crate::{ ADMIN_ROLE, GALAKI_WINNER};

#[derive(Accounts)]
#[instruction(new_operator_wallet: Pubkey)]
pub struct ChangeOperatorWallet<'info> {
    #[account(
        mut,
        seeds = [GALAKI_WINNER],
        bump = galaki_account.bump,
        constraint = galaki_account.has_admin(authority.key()) @ GalaKiErrors::AdminAccountInvalid,
    )]
    pub galaki_account: Box<Account<'info, Galaki>>,

    #[account(
        seeds = [ADMIN_ROLE, authority.key().as_ref()],
        bump = admin_account.bump,
        constraint = admin_account.has_authority(authority.key(), AuthRole::Admin ) == true @ GalaKiErrors::OnlyAdmin,
        constraint = admin_account.status == true @ GalaKiErrors::OnlyAdmin,
    )]
    pub admin_account:  Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn handle_change_operator_wallet(ctx: Context<ChangeOperatorWallet>, new_operator_wallet: Pubkey) -> Result<()> {
    let galaki_pda = &mut ctx.accounts.galaki_account;

    let auth = &ctx.accounts.authority;
    require!(galaki_pda.operator_wallet != new_operator_wallet, GalaKiErrors::OperatorWalletSameAsNewWallet);
    galaki_pda.change_operator_wallet(new_operator_wallet)?;


    //emit event
    emit!(ChangeOperatorWalletEvent{
        operator_wallet: new_operator_wallet,
        time: Clock::get()?.unix_timestamp,
        admin: auth.key(),
    });
    Ok(())
}