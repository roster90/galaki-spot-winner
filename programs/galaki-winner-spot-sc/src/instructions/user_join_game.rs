use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token:: AssociatedToken;
use solana_safe_math::SafeMath;
use crate::{ GalaKiErrors, GameProject};
use crate::GAME_PROJECT;


#[derive(Accounts)]
#[instruction(game_id: u64)]
pub struct PlayerJoinGame<'info> {

    #[account(
        mut,
        seeds = [GAME_PROJECT, game_id.to_be_bytes().as_ref()],
        bump = game_project_pda.bump,
        constraint = game_project_pda.is_active() == true @ GalaKiErrors::GameProjectInactive,
    )]
    pub game_project_pda: Box<Account<'info, GameProject>>,

    #[account(init_if_needed,  
        payer = payer, 
        associated_token::mint = token_mint, 
        associated_token::authority = game_project_pda)]

    pub game_ata: Account<'info, TokenAccount>,

    #[account(mut,
        constraint = user_ata.owner == payer.key() @GalaKiErrors::TokenAccountNotMatch,
        constraint = user_ata.mint == token_mint.key() @GalaKiErrors::TokenAccountNotMatch,
        constraint = user_ata.amount >0 @GalaKiErrors::InsufficientBalance,
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

pub fn handle_user_join_game(ctx: Context<PlayerJoinGame>, game_id: u64, number_sport: u32) -> Result<()> {
    let game_project_pda = &mut ctx.accounts.game_project_pda;
    let game_ata = &mut ctx.accounts.game_ata;
    let user_ata: &mut Account<TokenAccount> = &mut ctx.accounts.user_ata;
    let token_mint = &ctx.accounts.token_mint;
    let payer = &mut ctx.accounts.payer;
    let token_program = &ctx.accounts.token_program;

    let decimals = token_mint.decimals;

    let total_amount = ((game_project_pda.price_per_spot * number_sport) as u64).safe_mul(10u64.pow(decimals as u32))?; 
    //transfer token from user to game project
    msg!("total_amount: {:?}", total_amount);

    require!(user_ata.amount >= total_amount, GalaKiErrors::InsufficientBalance);
    msg!("transfer token from user to game project");
    
    let cpi_accounts = anchor_spl::token::Transfer {
        from: user_ata.to_account_info().clone(),
        to: game_ata.to_account_info().clone(),
        authority: payer.to_account_info().clone(),
    };
    let cpi_program = token_program.to_account_info();

    anchor_spl::token::transfer(CpiContext::new(cpi_program, cpi_accounts), total_amount)?;

    //calculate spot number for user

    //emit event

    Ok(())
}