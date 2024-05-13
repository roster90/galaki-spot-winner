
use anchor_lang::prelude::*;

use crate::{ GameProject, AuthorityRole, Galaki, AuthRole, GalaKiErrors};
use crate::{ OPERATOR_ROLE, GALAKI_WINNER, GAME_PROJECT};
#[derive(Accounts)]
pub struct CreateGameProject<'info> {

    #[account(
        mut,
        seeds = [GALAKI_WINNER],
        bump = galaki_account.bump, 
    )]

    pub galaki_account: Box<Account<'info, Galaki>>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 32 + 32 + 1,
        seeds = [GAME_PROJECT, galaki_account.counter.to_be_bytes().as_ref()],
        bump,
    )]
    pub game_project_account: Box<Account<'info, GameProject>>,
    #[account(
        seeds = [OPERATOR_ROLE, authority.key().as_ref()],
        bump =  operator_account.bump,
        constraint = operator_account.has_authority(authority.key(), AuthRole::Operator) == true @ GalaKiErrors::OnlyOperator,
    )]
    pub operator_account: Account<'info, AuthorityRole>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}