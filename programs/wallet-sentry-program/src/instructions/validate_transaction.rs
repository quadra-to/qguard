use anchor_lang::prelude::*;

use crate::WalletSentry;

#[derive(Accounts)]
pub struct ValidateTransaction<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, payer = payer, space = 8 + 32 + 32 + 32 + 1)]
    pub extension: Account<'info, WalletSentry>,

    /// CHECK: No writes or read here
    pub ext_add_metadata_authority: UncheckedAccount<'info>,

    /// CHECK: No writes or read here
    pub ext_modify_metadata_authority: UncheckedAccount<'info>,

    /// CHECK: No writes or read here
    pub ext_remove_metadata_authority: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handle_validate_transaction(
    ctx: Context<ValidateTransaction>
) -> Result<()> {
    Ok(())
}