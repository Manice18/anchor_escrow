use anchor_lang::prelude::*;

pub mod states;
pub mod contexts;

use contexts::*;

declare_id!("FTGFM3W8uxQz5hzxmSn9AzqveeeDeSD6siXiPePGUTai");

#[program]
pub mod escrow_new {
    use super::*;

    pub fn make(ctx: Context<Make>, 
        seed: u64, 
        // receive: u64, 
        deposit: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, 
            // receive,
             &ctx.bumps)?;
        ctx.accounts.deposit(deposit)
    }

    pub fn take(ctx: Context<Take>,amount:u64) -> Result<()> {
        // ctx.accounts.transfer()?;
        ctx.accounts.withdraw_and_close_vault(amount)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.withdraw_and_close_vault()
    }
}