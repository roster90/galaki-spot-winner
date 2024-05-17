
use solana_program::clock::Clock;
use solana_safe_math::SafeMath;


use crate::*;


#[derive(Accounts)]
pub struct GetRandomNumber<'info>{

    #[account(mut, signer)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


pub fn handle_get_random_number(ctx: Context<GetRandomNumber> ) -> Result<()> {
    // let slot_hashes = SlotHashes::from_account_info(&ctx.accounts.slot_hashes)?;

    // let (_, most_recent_hash) = slot_hashes.iter().next().ok_or(ProgramError::InvalidAccountData)?;
    let input = &ctx.accounts.payer.key().to_string();
    let user_number: u64 = input.parse().map_err(|_| ProgramError::InvalidArgument)?;
    msg!("number: {:?}", user_number);
    // let current_time = Clock::get()?.unix_timestamp;
    // msg!("blockhash_random_seed: {:?}", blockhash_random_seed);

    
    let slot = Clock::get()?.slot;
    msg!("slot: {:?}", slot);
    let current_time: u64 = Clock::get()?.unix_timestamp as u64;
    
    let random_number = xorshift(slot.safe_add(current_time)?);

    msg!("Random number: {:?}", random_number);
//    convert string to u64 
    // msg!("Random number: {:?}", blockhash_random_seed);
    Ok(())
}