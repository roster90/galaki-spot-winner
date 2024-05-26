use crate::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Token, TokenAccount, Mint}};

#[derive(Accounts)]
pub struct UserClaimReward<'info> {

    #[account(
        seeds = [GALAKI_GAME_WINNER],
        bump = game_pda.bump,
        constraint = game_pda.is_close == true @ GalaKiErrors::GameProjectInactive,
    )]
    pub game_pda: Account<'info, GalakiGame>,
    #[account(
        mut,
        constraint = game_ata.mint == reward_token_mint.key() @GalaKiErrors::TokenAccountNotMatch,
        constraint = game_ata.owner == game_pda.key() @GalaKiErrors::TokenAccountNotMatch,
    )]
    pub game_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [PLAYER, user.key().as_ref()],
        bump = user_pda.bump,
        constraint = user_pda.is_winner(game_pda.spot_winner) == true @ GalaKiErrors::UserNotWinner,
    )]
    
    pub user_pda: Box<Account<'info, Player>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = reward_token_mint, 
        associated_token::authority = user,
    )]
    pub user_ata: Account<'info, TokenAccount>,
    #[account(
        constraint = reward_token_mint.key() == game_pda.currency @GalaKiErrors::TokenAccountNotMatch,
    )] 
    pub reward_token_mint: Account<'info, Mint>,
    #[account(mut, signer)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handle_user_claim_reward(ctx: Context<UserClaimReward>) -> Result<()> {
    let game_ata: &Account<TokenAccount> = &ctx.accounts.game_ata;
    let game_pda = &ctx.accounts.game_pda;
    let user_pda = &mut ctx.accounts.user_pda;
    
    let reward_balance = game_ata.amount;

    require!(reward_balance > 0, GalaKiErrors::InsufficientBalance);


    //how to get reward balance

    let remaining = reward_balance;

    let seeds: &[&[u8]] = &[GALAKI_GAME_WINNER, &[ctx.accounts.game_pda.bump]];
    let signer = &seeds[..];
    _transfer_token( &TokenTransferParams {
        source: game_ata.to_account_info(),
        destination: ctx.accounts.user_ata.to_account_info(),
        authority: ctx.accounts.game_pda.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        authority_signer_seeds:signer,
        amount: remaining,
    })?;

    //update user pda
    user_pda.claim_reward(remaining)?;

    //emit event transfer token
    emit!(WithdrawTokenEvent{
        amount: remaining,
        from: game_pda.key(),
        to: user_pda.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });


    Ok(())
}