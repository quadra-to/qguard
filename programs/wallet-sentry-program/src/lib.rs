use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("68WTC9H8iAxGh89LPEavXf2vRcVK5jWU3v86SH28YuzV");

#[program]
pub mod wallet_sentry_program {
    use super::*;

    pub fn initialize_wallet_sentry(ctx: Context<InitializeWalletSentry>) -> Result<()> {
        handle_initialize_wallet_sentry(ctx)
    }

    pub fn validate_transaction(ctx: Context<ValidateTransaction>) -> Result<()> {
        handle_validate_transaction(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
