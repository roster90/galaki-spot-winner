use anchor_spl::{associated_token::AssociatedToken, token::{Token, TokenAccount, Mint}};
use crate::*;

#[derive(Accounts)]
pub struct SetupReward<'info> {
    #[account(
        mut,
        seeds = [GALAKI_GAME_WINNER],
        bump = galaki_game_pda.bump,
        constraint = galaki_game_pda.has_admin(authority.key()) @ GalaKiErrors::AdminAccountInvalid,
    )]
    pub galaki_game_pda: Box<Account<'info, GalakiGame>>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = reward_token_mint, 
        associated_token::authority = authority,
    )]
    pub galaki_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [ADMIN_ROLE, authority.key().as_ref()],
        bump = admin_account.bump,
        constraint = admin_account.has_authority(authority.key(), AuthRole::Admin ) == true @ GalaKiErrors::OnlyAdmin,
        constraint = admin_account.status == true @ GalaKiErrors::OnlyAdmin,
    )]
    pub admin_account:  Account<'info, AuthorityRole>,

    pub reward_token_mint: Account<'info, Mint>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>, 
}

pub fn handle_setup_reward(ctx: Context<SetupReward>, new_admin: Pubkey) -> Result<()> {
    let galaki_pda = &mut ctx.accounts.galaki_game_pda;

    Ok(())
}









