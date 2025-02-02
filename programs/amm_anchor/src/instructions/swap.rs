use anchor_lang::prelude::*;
use anchor_spl::{self, associated_token::AssociatedToken, token::TokenAccount};
use constant_product_curve::LiquidityPair;
use ConstantProduct::*;

use crate::state::Config;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Swap<'info> {
    
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config,
    )]
    pub vault_x: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    pub user_x: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    pub vault_y: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )]
    pub user_y: Account<'info, TokenAccount>,
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

}
impl<'info> Swap<'info> {
    pub fn swap(&mut self, is_x: bool, amount: u64, min: u64) -> Result<()> {
        
        let mut curve = ConstantProduct::init(
            x: self.vault_x.amount,
            y: self.vault_y.amount,
            l:self.vault_x.amount,
            self.config.fee,
            precision: None,
        ).unwrap();
        let p = match is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y
        };
        let res = curve.swap(p, amount, min).unwrap();

        self.deposit_tokens(is_x, res.deposit).unwrap();
        self.withdraw_tokens(is_x, res.withdraw).unwrap();
        Ok(())
    }
        
        
    pub fn deposit_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let (from,to) = match is_x {
            true => (self.user_x.to_account_info(), self. vault_x.to_account_info()),
            false => (self.user_y.to_account_info(), self. vault_y.to_account_info()),

        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_account = Transfer {
            from,
            to,
            authority: self.user.to_account_info()
        };
        let ctx = CpiContext::new(cpi_program, cpi_account);
        transfer(ctx, amount)?;
        Ok(())
    }
    pub fn withdraw_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let (from,to) = match is_x {
            true => (self.vault_x.to_account_info(), self. user_x.to_account_info()),
            false => (self.vault_y.to_account_info(), self. user_y.to_account_info()),

        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_account = Transfer {
            from,
            to,
            authority: self.config.to_account_info()
        };
        let seeds = &[
            &b"config"[..],
            &self.config.seed.to_le_bytes(),
            &[self.config.config_bump],
         ];
        
        let signer_seeds = &[&seeds[..]];
        let ctx = CpiContext::new_with_signer(cpi_program, cpi_account, signer_seeds);
        transfer(ctx, amount)?;
        Ok(())
    }
}