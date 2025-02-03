use anchor_lang::prelude::*;
 mod instructions;
 mod state;

use crate::instructions::*;
use crate::state::*;

declare_id!("AarinGuaZSrFwZXNwQPzQKRN3b4tTPQRx1KxiRAXWGif");

#[program]
pub mod amm_anchor {
    use crate::instructions::{Deposit, Initialize, Swap};

    use super::*;

    pub fn init(ctx: Context<Initialize>, seed: u64, fee: u16, authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.init(seed, fee, authority, &ctx.bumps)?;
        Ok(())
    }
    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64, is_x:bool) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y);
        ctx.accounts.deposit_tokens(is_x, amount)?;
        ctx.accounts.mint_lp(amount);
        Ok(())
    }
    pub fn swap(ctx: Context<Swap>, is_x:bool, amount: u64, min: u64) -> Result<()> {
        ctx.accounts.swap(is_x, amount, min)?;
        ctx.accounts.deposit_tokens(is_x, amount)?;
        ctx.accounts.withdraw_tokens(is_x, amount)?;
        Ok(())
    }
}

