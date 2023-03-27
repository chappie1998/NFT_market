use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token,
    token,
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mysolanaapp {
    use super::*;

    pub fn list(ctx: Context<ListNFT>) -> Result<()> {
        Ok(())
    }
}

// Transaction instructions
#[derive(Accounts)]
pub struct ListNFT<'info> {
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub seller_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub seller_wallet: Signer<'info>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, token::TokenAccount>,
    #[account(
        init,
        seeds = [b"marketplace", seller_wallet.key().as_ref()],
        payer = seller_token_account,
        space = 8+ 1 + 32 + 32 + 32 + 8 + 4 + 220, 
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, token::TokenAccount>,
}



// An account that goes inside a transaction instruction
#[account]
pub struct Escrow {
    pub is_initialized: bool,
    pub seller_pubkey: Pubkey,
    pub token_account_pubkey: Pubkey,
    pub mint_key: Pubkey,
    pub expected_amount: u64,
    pub bump: u8,
}