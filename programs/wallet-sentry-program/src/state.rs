use anchor_lang::prelude::*;

#[account]
pub struct WalletSentry {
    pub owner: Pubkey,
    pub bump: u8
}