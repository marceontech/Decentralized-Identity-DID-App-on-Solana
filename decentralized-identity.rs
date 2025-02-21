use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

declare_id!("FILL_WITH_YOUR_PROGRAM_ID");

#[program]
pub mod decentralized_identity {
    use super::*;

    pub fn register_did(ctx: Context<RegisterDID>, did_data: String) -> Result<()> {
        let did = &mut ctx.accounts.did_account;
        did.owner = *ctx.accounts.user.key;
        did.data = did_data;
        msg!("DID Registered: {}", did.data);
        Ok(())
    }

    pub fn verify_did(ctx: Context<VerifyDID>) -> Result<()> {
        let did = &ctx.accounts.did_account;
        require!(did.owner == *ctx.accounts.user.key, ErrorCode::Unauthorized);
        msg!("DID Verified: {}", did.data);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterDID<'info> {
    #[account(init, payer = user, space = 256)]
    pub did_account: Account<'info, DID>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyDID<'info> {
    #[account(has_one = owner)]
    pub did_account: Account<'info, DID>,
    pub user: Signer<'info>,
}

#[account]
pub struct DID {
    pub owner: Pubkey,
    pub data: String,
}
