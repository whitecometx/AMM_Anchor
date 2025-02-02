use anchor_lang::prelude::*;
 mod instructions;
 mod state;

use crate:: instructions::*;

declare_id!("AarinGuaZSrFwZXNwQPzQKRN3b4tTPQRx1KxiRAXWGif");

#[program]
pub mod amm_anchor {
    use crate::instructions::{Deposit, Init, Swap};

    use super::*;

    pub fn init(ctx: Context<Init>, seed: u64, fee: u16, authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.init(seed, fee, authority, &ctx.bumps)?;
        Ok(())
    }
    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y);
        ctx.accounts.deposit_tokens(is_x, amount)?;
        ctx.accounts.mint_lp(amount);
        Ok(())
    }
    pub fn swap(ctx: Context<Swap>) -> Result<()> {
        ctx.accounts.swap(is_x, amount, min)?;
        ctx.accounts.deposit_tokens(is_x, amount)?;
        ctx.accounts.withdraw_tokens(is_x, amount)?;
    }
}

