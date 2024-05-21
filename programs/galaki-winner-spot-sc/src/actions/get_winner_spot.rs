use solana_safe_math::SafeMath;

use crate::*;


#[derive(Accounts)]
pub struct GetWinnerSport<'info> {

    #[account(
        mut,
        seeds = [GALAKI_GAME_WINNER],
        bump = ganaki_game_pda.bump,
        constraint = ganaki_game_pda.get_status() == 2 @ GalaKiErrors::GameProjectInactive,
    )]
    pub ganaki_game_pda: Box<Account<'info, GalakiGame>>,
    #[account(
        seeds = [ADMIN_ROLE, authority.key().as_ref()],
        bump = admin_account.bump,
        constraint = admin_account.has_authority(authority.key(), AuthRole::Admin ) == true @ GalaKiErrors::OnlyAdmin,
        constraint = admin_account.status == true @ GalaKiErrors::OnlyAdmin,
    )]
    pub admin_account:  Account<'info, AuthorityRole>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_get_winner_sport(ctx: Context<GetWinnerSport>, _game_id: u64, seed_slot: u64) -> Result<()> {
    let ganaki_game_pda = &mut ctx.accounts.ganaki_game_pda;

    let slot = Clock::get()?.slot;
    let current_time = Clock::get()?.unix_timestamp as u64;
    
    let random_number = xorshift(slot.safe_add(current_time + seed_slot)?) % current_time;


    let random_index = (random_number % ganaki_game_pda.spot_numbers.len() as u64) as usize;

    let random_winner: u64 = ganaki_game_pda.spot_numbers[random_index];

    msg!("Random winner: {:?}", random_winner);
    ganaki_game_pda.set_winner(random_winner);


    //emit event


    Ok(())
}