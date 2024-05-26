

use anchor_spl::{associated_token::AssociatedToken, token::{Token, TokenAccount, Mint}};
use solana_safe_math::SafeMath;
use crate::*;

#[derive(Accounts)]
#[instruction(game_id: u64)]
pub struct UserParticipateGame<'info> {

    #[account(
        mut,
        seeds = [GALAKI_GAME_WINNER],
        bump = ganaki_game_pda.bump,
        constraint = ganaki_game_pda.get_status() == 1 @ GalaKiErrors::GameProjectInactive,
    )]
    pub ganaki_game_pda: Box<Account<'info, GalakiGame>>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 8 + 32 + 32 + 1,
        seeds = [PLAYER, ganaki_game_pda.key().as_ref(), user.key().as_ref()],
        bump 
    )]
    pub user_pda: Box<Account<'info, Player>>,

    #[account(
        mut,
        constraint = ganaki_game_ata.mint == token_mint.key() @GalaKiErrors::TokenAccountNotMatch,
    )]
    pub ganaki_game_ata: Account<'info, TokenAccount>,

    #[account(mut,
        constraint = user_ata.owner == user.key() @GalaKiErrors::TokenAccountNotMatch,
        constraint = user_ata.mint == token_mint.key() @GalaKiErrors::TokenAccountNotMatch,
        constraint = user_ata.amount >0 @GalaKiErrors::InsufficientBalance,
    )]  
    pub user_ata: Account<'info, TokenAccount>,
    #[account(
        constraint = token_mint.key() == ganaki_game_pda.currency @GalaKiErrors::TokenAccountNotMatch,
    )] 
    pub token_mint: Account<'info, Mint>,
    #[account(mut, signer)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

pub fn handle_participate_game(ctx: Context<UserParticipateGame>) -> Result<()> {
    let ganaki_game_pda = &mut ctx.accounts.ganaki_game_pda;
    let user_pda = &mut ctx.accounts.user_pda;

    //check number spot of users

    //check number spot of game project

    let game_ata = &mut ctx.accounts.ganaki_game_ata;
    let user_ata: &mut Account<TokenAccount> = &mut ctx.accounts.user_ata;
    let token_mint = &ctx.accounts.token_mint;
    let payer = &mut ctx.accounts.user;
    let token_program = &ctx.accounts.token_program;

    let decimals = token_mint.decimals;



    let participate_amount: u64 = (ganaki_game_pda.price_ticket  as u64).safe_mul(10u64.pow(decimals as u32)).unwrap(); 
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
    user_pda.initialize(&payer.key(), ctx.bumps.user_pda)?;

    // let recent_blockhashes =SlotHashes::slot_hashes(&ctx.accounts.system_program, 0, 1)?;

    
    // let current_time = Clock::get()?.unix_timestamp;
    // msg!("blockhash_random_seed: {:?}", blockhash_random_seed);

    let random_number = get_random_number();

    msg!("Random number: {:?}", random_number);
    require!(ganaki_game_pda.check_spot(random_number) == false, GalaKiErrors::RandomNumberInvalid);

    ganaki_game_pda.user_participated_amount(random_number)?;
    user_pda.add_spot_number(random_number);



    //emit event
    let clock = Clock::get().unwrap();
    emit!(UserParticipateEvent {
         game: ganaki_game_pda.key(),
         time: clock.unix_timestamp,
         user: payer.clone().key(),
         sport_numbers: random_number,
         slot: clock.slot,
    });

    Ok(())
}


