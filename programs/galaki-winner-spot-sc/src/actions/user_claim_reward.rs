use crate::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Token, TokenAccount, Mint}};

#[derive(Accounts)]
#[instruction(game_id: u64)]
pub struct UserClaimReward<'info> {

    #[account(
        seeds = [GAME_PROJECT, game_id.to_be_bytes().as_ref()],
        bump = game_project_pda.bump,
        constraint = game_project_pda.is_close == true @ GalaKiErrors::GameProjectInactive,
    )]
    pub game_project_pda: Account<'info, GameProject>,
    #[account(
        mut,
        constraint = game_project_ata.mint == token_mint.key() @GalaKiErrors::TokenAccountNotMatch,
        constraint = game_project_ata.owner == game_project_pda.key() @GalaKiErrors::TokenAccountNotMatch,
    )]
    pub game_project_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [PLAYER, game_id.to_be_bytes().as_ref(), payer.key().as_ref()],
        bump = user_pda.bump,
        constraint = user_pda.is_winner(game_project_pda.spot_winner) == true @ GalaKiErrors::UserNotWinner,
    )]
    
    pub user_pda: Box<Account<'info, Player>>,
    #[account(mut,
        constraint = user_ata.mint == token_mint.key() @GalaKiErrors::TokenAccountNotMatch,
    )]
    pub user_ata: Account<'info, TokenAccount>,
    #[account(
        constraint = token_mint.key() == game_project_pda.currency @GalaKiErrors::TokenAccountNotMatch,
    )] 
    pub token_mint: Account<'info, Mint>,
    #[account(mut, signer)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handle_user_claim_reward(ctx: Context<UserClaimReward>, game_id: u64) -> Result<()> {
    let game_project_pda = &ctx.accounts.game_project_pda;
    let game_ata = &ctx.accounts.game_project_ata;
    let user_ata: &mut Account<TokenAccount> = &mut ctx.accounts.user_ata;
    let token_mint = &ctx.accounts.token_mint;
    let payer = &ctx.accounts.payer;

    let user_balance = user_ata.amount;
    let game_balance = game_ata.amount;

    require!(user_balance > 0, GalaKiErrors::InsufficientBalance);
    require!(game_balance > 0, GalaKiErrors::InsufficientBalance);
    //handle winner withdraw rewards

    Ok(())
}