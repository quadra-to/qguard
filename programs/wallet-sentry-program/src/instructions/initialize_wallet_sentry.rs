use anchor_lang::prelude::*;

use crate::WalletSentry;

#[derive(Accounts)]
pub struct InitializeWalletSentry<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init, 
        payer = user, 
        space = 8 + 32 + 1,
        seeds = [
            b"quadrato_wallet_sentry",
            user.key().as_ref()
        ], 
        bump
    )]
    pub wallet_sentry: Account<'info, WalletSentry>,
    
    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_wallet_sentry(
    ctx: Context<InitializeWalletSentry>
) -> Result<()> {
    Ok(())
}