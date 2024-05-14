use crate::*;


#[event]
pub struct VrfClientCreated {
    pub vrf_client: Pubkey,
    pub max_result: u64,
    pub timestamp: i64,
}


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitClientParams {
    pub max_result: u64,
}

impl InitClient<'_>  {
    pub fn validate(&self, _ctx: &Context<Self>, params: &InitClientParams) -> Result<()> {
        msg!("init_client validate");
        if params.max_result > 1337 {
            return Err(error!(VrfClientErrorCode::MaxResultExceedsMaximum));
        }

        Ok(())
    }

    pub fn actuate(ctx: &Context<Self>, params: &InitClientParams) -> Result<()> {
        msg!("init_client actuate");

        let mut state = ctx.accounts.state.load_init()?;
        *state = VrfClientState::default();
        state.bump = ctx.bumps.get("state").unwrap().clone();
        state.vrf = ctx.accounts.vrf.key();

        if params.max_result == 0 {
            state.max_result = 1337;
        } else {
            state.max_result = params.max_result;
        }

        emit!(VrfClientCreated{
            vrf_client: ctx.accounts.state.key(),
            max_result: params.max_result,
            timestamp: Clock::get().unwrap().unix_timestamp
        });

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(params: InitClientParams)]
pub struct InitClient<'info> {
    #[account(
        init,
        seeds = [
            STATE_SEED,
            vrf.key().as_ref()
        ],
        payer = payer,
        space = 8 + std::mem::size_of::<VrfClientState>(),
        bump,
    )]
    pub state: AccountLoader<'info, VrfClientState>,
    #[account(
        constraint = vrf.load()?.authority == state.key() @ VrfClientErrorCode::InvalidVrfAuthorityError
    )]
    pub vrf: AccountLoader<'info, VrfAccountData>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[repr(packed)]
#[account(zero_copy(unsafe))]
#[derive(Default)]
pub struct VrfClientState {
    pub bump: u8,
    pub max_result: u64,
    pub result_buffer: [u8; 32],
    pub result: u128,
    pub timestamp: i64,
    pub vrf: Pubkey,
}