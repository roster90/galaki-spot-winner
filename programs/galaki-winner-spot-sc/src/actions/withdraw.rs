use anchor_spl::{associated_token::AssociatedToken, token::{Token, TokenAccount, Mint}};


use crate::*;


#[derive(Accounts)]
pub struct WithdrawSpl<'info>{
    #[account(
        seeds = [GALAKI_GAME_WINNER],
        bump = galaki_game_pda.bump, 
    )]
    pub galaki_game_pda: Box<Account<'info, GalakiGame>>,
    
    #[account(
        seeds = [ADMIN_ROLE, authority.key().as_ref()],
        bump = admin_account.bump,
        constraint = admin_account.has_authority(authority.key(), AuthRole::Admin ) == true @ GalaKiErrors::OnlyAdmin,
        constraint = galaki_game_pda.has_admin(authority.key()) @ GalaKiErrors::OnlyAdmin,
        constraint = admin_account.status == true @ GalaKiErrors::OnlyAdmin,
    )]
    pub admin_account:  Account<'info, AuthorityRole>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = token_mint, 
        associated_token::authority = authority,
    )]
    pub admin_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = galaki_game_pda,
    )]
    pub galaki_game_ata: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>, 

}


pub fn withdraw_spl_handle(ctx: Context<WithdrawSpl>, amount: u64) -> Result<()> {

    let ganaki_game_pda = &mut ctx.accounts.galaki_game_pda;
    let token_mint = &ctx.accounts.token_mint;
   
    let to_ata = &mut ctx.accounts.admin_ata;
    let from_ata = &mut ctx.accounts.galaki_game_ata;

    msg!("Tranfer {:} {:} to admin", amount, token_mint.key());
    require!(from_ata.amount >= amount, GalaKiErrors::InsufficientAmount);


    let seeds: &[&[u8]] = &[GALAKI_GAME_WINNER, &[ganaki_game_pda.bump]];

    _transfer_token(
        &TokenTransferParams {
            source: from_ata.to_account_info(),
            destination: to_ata.to_account_info(),
            authority: ganaki_game_pda.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            authority_signer_seeds: seeds,
            amount,
        }
    )?;
    //emit event withdraw token
    emit!(WithdrawTokenEvent{
        from: ganaki_game_pda.key(),
        to: ctx.accounts.authority.key(),
        amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}



