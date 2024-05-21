use crate::*;


#[derive(Accounts)]
pub struct AdminIntrusion<'info> {
    #[account(
        seeds = [GALAKI_GAME_WINNER],
        bump = galaki_game_pda.bump,
        constraint = galaki_game_pda.has_admin(authority.key()) @ GalaKiErrors::AdminAccountInvalid,
        constraint = galaki_game_pdaget_status() == 2 @ GalaKiErrors::GameProjectInactive,
    )]
    pub galaki_game_pda: Box<Account<'info, GalakiGame>>,

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

impl AdminAction <'_> {
    pub fn handle_remove_admin(&self, _ctx: &Context<Self>, admin: Pubkey) -> Result<()> {
        let galaki_pda = &mut ctx.accounts.galaki_account;
        let admin_account = &mut ctx.accounts.admin_account;
        let authority = &ctx.accounts.authority;
        galaki_pda.remove_admin(admin, authority.key())?;
        admin_account.remove_admin(admin, authority.key())?;
        Ok(())
    }


}