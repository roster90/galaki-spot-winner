use anchor_lang::prelude::*;

use crate::{ AuthorityRole, Galaki, AuthRole, GalaKiErrors};
use crate::{ ADMIN_ROLE, GALAKI_WINNER, OPERATOR_ROLE};

#[derive(Accounts)]
#[instruction(new_admin: Pubkey)]
pub struct SetAdmin<'info> {
    #[account(
        mut,
        seeds = [GALAKI_WINNER],
        bump = galaki_account.bump,
        constraint = galaki_account.has_admin(authority.key()) @ GalaKiErrors::AdminAccountInvalid,
    )]
    pub galaki_account: Box<Account<'info, Galaki>>,

    #[account(
        init_if_needed,
        space = 60,
        payer = authority,
        seeds = [ADMIN_ROLE, new_admin.as_ref()], 
        bump,
    )]
    pub new_admin_account:  Account<'info, AuthorityRole>,

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

pub fn handle_set_admin(ctx: Context<SetAdmin>, new_admin: Pubkey) -> Result<()> {
    let galaki_pda = &mut ctx.accounts.galaki_account;
    let new_admin_account = &mut ctx.accounts.new_admin_account;
    galaki_pda.set_admin(new_admin)?;
    new_admin_account.initialize(&new_admin, ctx.bumps.new_admin_account, AuthRole::Admin)?;
    Ok(())
}

#[derive(Accounts)]
#[instruction(admin: Pubkey)]
pub struct RemoveAdmin<'info> {
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

    #[account(
        seeds = [ADMIN_ROLE, admin.as_ref()],
        bump = admin_to_remove.bump,
        constraint = admin_account.has_authority(admin, AuthRole::Admin ) == true @ GalaKiErrors::OnlyAdmin,
        constraint = admin_account.status == true @ GalaKiErrors::OnlyAdmin,
    )]
    pub admin_to_remove:  Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn handle_remove_admin(ctx: Context<RemoveAdmin>, admin: Pubkey) -> Result<()> {
    let galaki_pda = &mut ctx.accounts.galaki_account;
    let admin_account = &mut ctx.accounts.admin_account;
    galaki_pda.remove_admin(admin)?;
    admin_account.set_status_account(false);
    Ok(())
}

#[derive(Accounts)]
#[instruction(new_operator: Pubkey)]
pub struct SetOperator<'info> {
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

    #[account(
        init_if_needed,
        space = 60,
        payer = authority,
        seeds = [OPERATOR_ROLE, new_operator.as_ref()],
        bump)]
    pub operator_account:  Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn handle_set_operator(ctx: Context<SetOperator>, new_operator: Pubkey) -> Result<()> {
    let operator_pda = &mut ctx.accounts.operator_account;
    operator_pda.initialize(&new_operator, ctx.bumps.operator_account, AuthRole::Operator)?;
    Ok(())
}


#[derive(Accounts)]
#[instruction(operator: Pubkey)]
pub struct RemoveOperator<'info> {
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

    #[account(
        seeds = [OPERATOR_ROLE, operator.as_ref()],
        bump = operator_account.bump,
        constraint = operator_account.has_authority(operator, AuthRole::Operator ) == true @ GalaKiErrors::OnlyOperator,
        constraint = operator_account.status == true @ GalaKiErrors::OnlyOperator,
    )]
    pub operator_account:  Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn handle_remove_operator(ctx: Context<RemoveOperator>, _: Pubkey) -> Result<()> {
    let operator_pda = &mut ctx.accounts.operator_account;
    operator_pda.set_status_account(false);
    Ok(())
}





