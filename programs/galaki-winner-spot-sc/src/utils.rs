

use anchor_lang::prelude::*;









pub fn _transfer_token_from_ido<'a>(data: &'a TokenTransferParams) -> Result<()> {
    let transfer_instruction = anchor_spl::token::Transfer {
        from: data.source.to_account_info(),
        to: data.destination.to_account_info(),
        authority: data.authority.to_account_info(),
    };
    let cpi_program = data.token_program.to_account_info();
    let signer = &[data.authority_signer_seeds];
    let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction).with_signer(signer);
    anchor_spl::token::transfer(cpi_ctx, data.amount)?;
    Ok(())
}

pub struct TokenTransferParams<'a: 'b, 'b> {
    /// source
    /// CHECK: account checked in CPI
    pub source: AccountInfo<'a>,
    /// destination
    /// CHECK: account checked in CPI
    pub destination: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// authority
    /// CHECK: account checked in CPI
    pub authority: AccountInfo<'a>,
    /// authority_signer_seeds
    pub authority_signer_seeds: &'b [&'b [u8]],
    /// token_program
    /// CHECK: account checked in CPI
    pub token_program: AccountInfo<'a>,
}

