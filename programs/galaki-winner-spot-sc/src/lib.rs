use anchor_lang::prelude::*;


declare_id!("3D4YaviBjiz7DXDeGtKRerkU3q2rNrDhvqPd5sRTE8QB");

pub mod types;
pub mod state;
pub mod error;
pub mod constants;
pub mod events;
pub mod actions;
pub mod utils;


pub use constants::*;
pub use error::*;
pub use events::*;
pub use state::*;
pub use types::*;
pub use actions::*;
pub use utils::*;


#[program]
pub mod galaki_winner_spot_sc {



    use super::*;
   

    //owner fn
    pub fn initialize(ctx: Context<InitializeGanakiGame>, params: GameInitParams) -> Result<()> {
        create_galaki_game::handle_initialize_galaki(ctx, params)
    }

    //admin fn
    pub fn set_admin(ctx: Context<SetAdmin>, new_admin: Pubkey) -> Result<()> {
        set_auth_role::handle_set_admin(ctx, new_admin)
    }



    pub fn participate(ctx: Context<UserParticipateGame>) -> Result<()> {
        user_participate_game::handle_participate_game(ctx)
    }

    pub fn get_winner_spot(ctx: Context<GetWinnerSport>, seed: u64) -> Result<()> {
        get_winner_spot::handle_get_winner_sport(ctx, seed)
    }

    pub fn withdraw(ctx: Context<WithdrawSpl>, amount: u64) -> Result<()> {
        withdraw::withdraw_spl_handle(ctx, amount)
    }

 

    // #[access_control(ctx.accounts.validate(&ctx, &params))]
    // pub fn init_client(ctx: Context<InitClient>, params: InitClientParams) -> Result<()> {
    //     // InitClient::actuate(&ctx, &params)
    // }



    // pub fn request_randomness(ctx: Context<RequestRandomness>, params: RequestRandomness) -> Result<()> {
    //     let max_request = u32::MAX;
    //     let vrf_client = ctx.accounts.vrf_client.key();
    //     let timestamp = Clock::get()?.unix_timestamp;

    //     let switchboard_program = ctx.accounts.switchboard_program.to_account_info();
    //     let vrf_request_randomness = VrfRequestRandomness {
    //     authority: ctx.accounts.state.to_account_info(),
    //     vrf: ctx.accounts.vrf.to_account_info(),
    //     oracle_queue: ctx.accounts.oracle_queue.to_account_info(),
    //     queue_authority: ctx.accounts.queue_authority.to_account_info(),
    //     data_buffer: ctx.accounts.data_buffer.to_account_info(),
    //     permission: ctx.accounts.permission.to_account_info(),
    //     escrow: ctx.accounts.escrow.clone(),
    //     payer_wallet: ctx.accounts.payer_wallet.clone(),
    //     payer_authority: ctx.accounts.payer_authority.to_account_info(),
    //     recent_blockhashes: ctx.accounts.recent_blockhashes.to_account_info(),
    //     program_state: ctx.accounts.program_state.to_account_info(),
    //     token_program: ctx.accounts.token_program.to_account_info(),
    //     };

    //         let vrf_key = ctx.accounts.vrf.key.clone();
    //         let authority_key = ctx.accounts.authority.key.clone();

    //         let state_seeds: &[&[&[u8]]] = &[&[
    //         &STATE_SEED,
    //         vrf_key.as_ref(),
    //         authority_key.as_ref(),
    //         &[bump],
    //         ]];
    //         msg!("requesting randomness");
    //         vrf_request_randomness.invoke_signed(
    //         switchboard_program,
    //         params.switchboard_state_bump,
    //         params.permission_bump,
    //         state_seeds,
    //         )?;
    //     Ok(())
    // }



}



// #[derive(Accounts)]
// #[instruction(params: RequestRandomnessParams)] // rpc parameters hint
// pub struct RequestRandomness<'info> {
//     #[account(
//         mut,
//         seeds = [
//             STATE_SEED,
//             vrf.key().as_ref(),
//         ],
//         bump = state.load()?.bump,
//         has_one = vrf @ VrfClientErrorCode::InvalidVrfAccount
//     )]
//     pub state: AccountLoader<'info, VrfClientState>,

//     // SWITCHBOARD ACCOUNTS
//     #[account(mut,
//         has_one = escrow
//     )]
//     pub vrf: AccountLoader<'info, VrfAccountData>,
//     #[account(mut,
//         has_one = data_buffer
//     )]
//     pub oracle_queue: AccountLoader<'info, OracleQueueAccountData>,
//     /// CHECK:
//     #[account(mut,
//         constraint =
//             oracle_queue.load()?.authority == queue_authority.key()
//     )]
//     pub queue_authority: UncheckedAccount<'info>,
//     /// CHECK
//     #[account(mut)]
//     pub data_buffer: AccountInfo<'info>,
//     #[account(mut)]
//     pub permission: AccountLoader<'info, PermissionAccountData>,
//     #[account(mut,
//         constraint =
//             escrow.owner == program_state.key()
//             && escrow.mint == program_state.load()?.token_mint
//     )]
//     pub escrow: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub program_state: AccountLoader<'info, SbState>,
//     /// CHECK:
//     #[account(
//         address = *vrf.to_account_info().owner,
//         constraint = switchboard_program.executable == true
//     )]
//     pub switchboard_program: AccountInfo<'info>,

//     // PAYER ACCOUNTS
//     #[account(mut,
//         constraint =
//             payer_wallet.owner == payer_authority.key()
//             && escrow.mint == program_state.load()?.token_mint
//     )]
//     pub payer_wallet: Account<'info, TokenAccount>,
//     /// CHECK:
//     #[account(signer)]
//     pub payer_authority: AccountInfo<'info>,

//     // SYSTEM ACCOUNTS
//     /// CHECK:
//     #[account(address = solana_program::sysvar::recent_blockhashes::ID)]
//     pub recent_blockhashes: AccountInfo<'info>,
//     pub token_program: Program<'info, Token>,
// }

// #[derive(Clone, AnchorSerialize, AnchorDeserialize)]
// pub struct RequestRandomnessParams {
//     pub permission_bump: u8,
//     pub switchboard_state_bump: u8,
// }

// impl RequestRandomness<'_> {
//     pub fn validate(&self, _ctx: &Context<Self>, _params: &RequestRandomnessParams) -> Result<()> {
//         Ok(())
//     }

//     pub fn actuate(ctx: &Context<Self>, params: &RequestRandomnessParams) -> Result<()> {
//         let client_state = ctx.accounts.state.load()?;
//         let bump = client_state.bump.clone();
//         let max_result = client_state.max_result;
//         drop(client_state);

//         let switchboard_program = ctx.accounts.switchboard_program.to_account_info();

//         let vrf_request_randomness = VrfRequestRandomness {
//             authority: ctx.accounts.state.to_account_info(),
//             vrf: ctx.accounts.vrf.to_account_info(),
//             oracle_queue: ctx.accounts.oracle_queue.to_account_info(),
//             queue_authority: ctx.accounts.queue_authority.to_account_info(),
//             data_buffer: ctx.accounts.data_buffer.to_account_info(),
//             permission: ctx.accounts.permission.to_account_info(),
//             escrow: ctx.accounts.escrow.clone(),
//             payer_wallet: ctx.accounts.payer_wallet.clone(),
//             payer_authority: ctx.accounts.payer_authority.to_account_info(),
//             recent_blockhashes: ctx.accounts.recent_blockhashes.to_account_info(),
//             program_state: ctx.accounts.program_state.to_account_info(),
//             token_program: ctx.accounts.token_program.to_account_info(),
//         };

//         let vrf_key = ctx.accounts.vrf.key();
//         let state_seeds: &[&[&[u8]]] = &[&[
//             &STATE_SEED,
//             vrf_key.as_ref(),
//             &[bump],
//         ]];

//         msg!("requesting randomness");
//         vrf_request_randomness.invoke_signed(
//             switchboard_program,
//             params.switchboard_state_bump,
//             params.permission_bump,
//             state_seeds,
//         )?;

//         let mut client_state = ctx.accounts.state.load_mut()?;
//         client_state.result = 0;

//         emit!(RandomnessRequested{
//             vrf_client: ctx.accounts.state.key(),
//             max_result: max_result,
//             timestamp: Clock::get().unwrap().unix_timestamp
//         });

//         msg!("randomness requested successfully");
//         Ok(())
//     }
// }


