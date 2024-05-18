
use crate::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Token, TokenAccount, Mint}};


#[derive(Accounts)]
#[instruction(data: GameInitParams)]
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
    #[account(init_if_needed,  
        payer = authority, 
        associated_token::mint = token_mint, 
        associated_token::authority = game_project_account)]
    pub game_project_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [OPERATOR_ROLE, authority.key().as_ref()],
        bump = operator_account.bump,
        constraint = operator_account.has_authority(authority.key(), AuthRole::Operator) == true @ GalaKiErrors::OnlyOperator,
    )]
    pub operator_account: Account<'info, AuthorityRole>,

    pub token_mint: Account<'info, Mint>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

pub fn handle_create_game_project(ctx: Context<CreateGameProject>, data: GameInitParams) -> Result<()> {

    let galaki_pda = &mut ctx.accounts.galaki_account;
    let game_project_pda = &mut ctx.accounts.game_project_account;

    let start_time = data.start_time;


    let current_time = Clock::get().unwrap().unix_timestamp;
    

    require!(start_time > current_time, GalaKiErrors::TimeInvalid);

    let game_id = galaki_pda.get_counter();

    game_project_pda.initialize(game_id, 
        data.currency,
        data.price_ticket, 
        data.max_ticket,
        data.max_ticket_per_user,
        start_time,
        data.duration,
    ctx.bumps.game_project_account )?;
    
    galaki_pda.auto_increase_game();

    //emit event create game project
    emit!(CreateGameEvent{
        game_account: game_project_pda.key(),
        id: game_id,
        currency: data.currency,
        start_time: start_time,
        end_time: start_time + data.duration,
    });


    Ok(())
}