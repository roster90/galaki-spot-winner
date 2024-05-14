
use crate::*;
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
        bump = operator_account.bump,
        constraint = operator_account.has_authority(authority.key(), AuthRole::Operator) == true @ GalaKiErrors::OnlyOperator,
    )]
    pub operator_account: Account<'info, AuthorityRole>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handle_create_game_project(ctx: Context<CreateGameProject>, data: GameInitParams) -> Result<()> {

    let galaki_pda = &mut ctx.accounts.galaki_account;
    let game_project_pda = &mut ctx.accounts.game_project_account;

    let start_time = data.start_time;
    let end_time = data.end_time;
    let current_time = Clock::get()?.unix_timestamp;
    require!(start_time > end_time || start_time < current_time, GalaKiErrors::TimeInvalid);

    let game_id = galaki_pda.get_counter();

    game_project_pda.initialize(game_id, data.currency,data.price_per_spot, start_time, end_time, ctx.bumps.game_project_account )?;
    
    galaki_pda.auto_increase_game();

    //emit event create game project
    emit!(CreateGameEvent{
        game_account: game_project_pda.key(),
        id: game_id,
        currency: data.currency,
        start_time: start_time,
        end_time: end_time,
    });


    Ok(())
}