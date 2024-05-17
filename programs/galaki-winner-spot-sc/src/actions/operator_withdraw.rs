use anchor_spl::{associated_token::AssociatedToken, token::{Token, TokenAccount, Mint}};


use crate::*;


#[derive(Accounts)]
#[instruction(game_id: u64, seed_slot: u64)]
pub struct WithdrawToken<'info> {

    #[account(
        seeds = [GALAKI_WINNER],
        bump = galaki_pda.bump, 
    )]
    pub galaki_pda: Box<Account<'info, Galaki>>,
    #[account(
        seeds = [GAME_PROJECT, game_id.to_be_bytes().as_ref()],
        bump = game_project_pda.bump,
        constraint = game_project_pda.is_active() == true @ GalaKiErrors::GameProjectInactive,
    )]
    pub game_project_pda: Box<Account<'info, GameProject>>,
    #[account(
        mut,
        constraint = game_project_ata.mint == token_mint.key() @GalaKiErrors::TokenAccountNotMatch,
        constraint = game_project_ata.owner == game_project_pda.key() @GalaKiErrors::TokenAccountNotMatch,
    )]
    pub game_project_ata: Account<'info, TokenAccount>,
    #[account(
        constraint = token_mint.key() == game_project_pda.currency @GalaKiErrors::TokenAccountNotMatch,
    )] 

    #[account(
        seeds = [OPERATOR_ROLE, authority.key().as_ref()],
        bump = operator_pda.bump,
        constraint = operator_pda.has_authority(authority.key(), AuthRole::Operator) == true @ GalaKiErrors::OnlyOperator,
    )]
    pub operator_pda: Account<'info, AuthorityRole>,

    pub token_mint: Account<'info, Mint>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

pub fn handle_withdraw_token(ctx: Context<WithdrawToken>, ) -> Result<()> {
    let game_project_pda = &mut ctx.accounts.game_project_pda;




    



    //emit event

    Ok(())
}