

use anchor_spl::{associated_token::AssociatedToken, token::{Token, TokenAccount, Mint}};
use solana_safe_math::SafeMath;
use crate::*;

#[derive(Accounts)]
#[instruction(game_id: u64)]
pub struct UserParticipateGame<'info> {

    #[account(
        mut,
        seeds = [GAME_PROJECT, game_id.to_be_bytes().as_ref()],
        bump = game_project_pda.bump,
        constraint = game_project_pda.get_status() == 1 @ GalaKiErrors::GameProjectInactive,
    )]
    pub game_project_pda: Box<Account<'info, GameProject>>,

    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + 32 + 8 + 32 + 32 + 1,
        seeds = [GAME_PROJECT, game_id.to_be_bytes().as_ref(), payer.key().as_ref()],
        bump 
    )]
    pub user_pda: Box<Account<'info, Player>>,

    #[account(
        mut,
        constraint = game_project_ata.mint == token_mint.key() @GalaKiErrors::TokenAccountNotMatch,
    )]

    pub game_project_ata: Account<'info, TokenAccount>,

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

pub fn handle_participate_game(ctx: Context<UserParticipateGame>, game_id: u64) -> Result<()> {
    let game_project_pda = &mut ctx.accounts.game_project_pda;
    let user_pda = &mut ctx.accounts.user_pda;

    //check number spot of users

    //check number spot of game project

    let game_ata = &mut ctx.accounts.game_project_ata;
    let user_ata: &mut Account<TokenAccount> = &mut ctx.accounts.user_ata;
    let token_mint = &ctx.accounts.token_mint;
    let payer = &mut ctx.accounts.payer;
    let token_program = &ctx.accounts.token_program;

    let decimals = token_mint.decimals;



    let participate_amount: u64 = (game_project_pda.price_ticket  as u64).safe_mul(10u64.pow(decimals as u32)).unwrap(); 
    //transfer token from user to game project
    msg!("participate_amount: {:?}", participate_amount);

    require!(user_ata.amount >= participate_amount, GalaKiErrors::InsufficientBalance);
    msg!("transfer token from user to game project");
    
    let cpi_accounts = anchor_spl::token::Transfer {
        from: user_ata.to_account_info().clone(),
        to: game_ata.to_account_info().clone(),
        authority: payer.to_account_info().clone(),
    };
    let cpi_program = token_program.to_account_info();

    anchor_spl::token::transfer(CpiContext::new(cpi_program, cpi_accounts), participate_amount)?;

    //init user spot
    user_pda.initialize(&payer.key(), ctx.bumps.user_pda, game_id)?;

    // let recent_blockhashes =SlotHashes::slot_hashes(&ctx.accounts.system_program, 0, 1)?;

    
    // let current_time = Clock::get()?.unix_timestamp;
    // msg!("blockhash_random_seed: {:?}", blockhash_random_seed);

    let random_number = get_random_number(game_id);

    msg!("Random number: {:?}", random_number);
    require!(game_project_pda.check_spot(random_number) == false, GalaKiErrors::RandomNumberInvalid);

    game_project_pda.user_participated_amount(random_number)?;
    user_pda.add_spot_number(random_number);


    //emit event
    emit!(UserParticipateEvent {
         game_id: game_id,
         time: Clock::get()?.unix_timestamp,
         user: payer.clone().key(),
         sport_numbers: random_number,
    });

    Ok(())
}


