use anchor_spl::{associated_token::AssociatedToken, token::{Token, TokenAccount, Mint}};


use crate::*;


#[derive(Accounts)]
#[instruction(game_id: u64)]
pub struct OperatorWithdrawSpl<'info>{
    #[account(
        seeds = [GALAKI_WINNER],
        bump = galaki_pda.bump, 
    )]
    pub galaki_pda: Box<Account<'info, Galaki>>,
    
    #[account(
        seeds = [GAME_PROJECT, game_id.to_be_bytes().as_ref()],
        bump = game_project_pda.bump,
        constraint = game_project_pda.get_status() == 2 @ GalaKiErrors::GameProjectInactive,
    )]
    pub game_project_pda: Box<Account<'info, GameProject>>,
    #[account(
        seeds = [OPERATOR_ROLE, authority.key().as_ref()],
        bump = operator_account.bump,
        constraint = operator_account.has_authority(authority.key(), AuthRole::Operator) == true @ GalaKiErrors::OnlyOperator,
    )]
    pub operator_account: Account<'info, AuthorityRole>,


     /// CHECK: Create a new associated token account for the operator account
     pub operator_wallet: AccountInfo<'info>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = token_mint, 
        associated_token::authority = operator_wallet,
    )]
    pub operator_wallet_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = game_project_pda,
    )]
    pub game_project_ata: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>, 

}


pub fn withdraw_spl_handle(ctx: Context<OperatorWithdrawSpl>, amount: u64) -> Result<()> {

    let operater_wallet = &ctx.accounts.galaki_pda;
    require!(operater_wallet.operator_wallet == ctx.accounts.operator_wallet.key(), GalaKiErrors::InvalidOperatorWallet);
    let game_pda = &mut ctx.accounts.game_project_pda;
    let token_mint = &ctx.accounts.token_mint;
   
    let to_ata = &mut ctx.accounts.operator_wallet_ata;
    let from_ata = &mut ctx.accounts.game_project_ata;

    msg!("Tranfer {:} {:} to admin", amount, token_mint.key());
    require!(from_ata.amount >= amount, GalaKiErrors::InsufficientAmount);

    let game_id_bytes = game_pda.id.to_be_bytes();
    let seeds: &[&[u8]] = &[GAME_PROJECT, game_id_bytes.as_ref(), &[game_pda.bump]];

    _transfer_token(
        &TokenTransferParams {
            source: from_ata.to_account_info(),
            destination: to_ata.to_account_info(),
            authority: game_pda.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            authority_signer_seeds: seeds,
            amount,
        }
    )?;
    //emit event withdraw token
    emit!(WithdrawTokenEvent{
        game_id: game_pda.id,
        from: game_pda.key(),
        to: ctx.accounts.operator_wallet_ata.key(),
        amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}



