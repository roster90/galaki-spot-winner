

use std::ops::Add;

use anchor_lang::prelude::*;
use solana_safe_math::SafeMath;


pub fn get_random_number() -> u64{
    let slot = Clock::get().unwrap().slot;
    let current_time = Clock::get().unwrap().unix_timestamp as u64;
    xorshift(slot.safe_add(current_time.add(1)).unwrap()) % current_time
}


fn _get_random_numbers( number_tickets: u16) -> Vec<u64> {
    let slot = Clock::get().unwrap().slot;
    let current_time = Clock::get().unwrap().unix_timestamp as u64;
    let mut random_numbers = Vec::new();
    for i in 0..number_tickets {
        let random_number = xorshift(slot.safe_add(current_time.add(i as u64)).unwrap()) % current_time;
        random_numbers.push(random_number);

    }
    random_numbers
}


pub fn xorshift(seed: u64) -> u64 {
    let mut x = seed;
    x ^= x << 12;
    x ^= x >> 8;
    x ^= x << 11;
    x
}






pub fn _transfer_token<'a>(data: &'a TokenTransferParams) -> Result<()> {
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

