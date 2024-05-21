
use crate::*;


#[derive(Accounts)]
#[instruction(data: GameInitParams)]
pub struct InitializeGanakiGame<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 4112,
        seeds = [GALAKI_GAME_WINNER],
        bump,
    )]
    pub galaki_account: Account<'info, GalakiGame>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 32 + 32 + 1,
        seeds = [ADMIN_ROLE, authority.key().as_ref() ],
        bump,
    )]
    pub admin_account: Account<'info, AuthorityRole>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_galaki(ctx: Context<InitializeGanakiGame>,data: GameInitParams) -> Result<()> {
    let galaki_game = &mut ctx.accounts.galaki_account;
    let admin_role = &mut ctx.accounts.admin_account;
    let authority = &ctx.accounts.authority;
    
    galaki_game.initialize(data.currency, data.price_ticket, data.max_ticket, data.max_ticket_per_user, data.start_time, data.duration, authority.key(), ctx.bumps.galaki_account)?;

    admin_role.initialize(&authority.key(), ctx.bumps.admin_account, AuthRole::Admin)?;

    Ok(())
}