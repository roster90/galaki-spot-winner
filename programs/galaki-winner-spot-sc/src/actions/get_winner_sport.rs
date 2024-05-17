use solana_safe_math::SafeMath;

use crate::*;


#[derive(Accounts)]
#[instruction(game_id: u64, seed_slot: u64)]
pub struct GetWinnerSport<'info> {

    #[account(
        mut,
        seeds = [GAME_PROJECT, game_id.to_be_bytes().as_ref()],
        bump = game_project_pda.bump,
        constraint = game_project_pda.is_active() == true @ GalaKiErrors::GameProjectInactive,
    )]
    pub game_project_pda: Box<Account<'info, GameProject>>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_get_winner_sport(ctx: Context<GetWinnerSport>, _: u64, seed_slot: u64) -> Result<()> {
    let game_project_pda = &mut ctx.accounts.game_project_pda;



    let slot = Clock::get()?.slot;
    let current_time = Clock::get()?.unix_timestamp as u64;
    
    let random_number = xorshift(slot.safe_add(current_time + seed_slot)?);
    msg!("Random number: {:?}", random_number);

    let random_index = (random_number % game_project_pda.spot_numbers.len() as u64) as usize;
    let random_element = game_project_pda.spot_numbers[random_index];
    game_project_pda.set_winner(random_element);


    //emit event

    Ok(())
}