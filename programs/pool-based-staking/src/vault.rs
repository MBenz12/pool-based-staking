use crate::errors::*;
use crate::ins::*;

use anchor_lang::prelude::*;

pub fn handle_vault_initialization(
  ctx: Context<InitializeVault>,
  creator_address: Pubkey,
  payout_interval: u64,
  payout_amount: u64,
  stake_fee: u64,
  unstake_fee: u64,
) -> Result<()> {
  let mut vault = ctx.accounts.vault.load_init()?;

  vault.authority = ctx.accounts.authority.key();
  vault.creator_address = creator_address;
  vault.payout_interval = payout_interval;
  vault.payout_amount = payout_amount;
  vault.stake_fee = stake_fee;
  vault.unstake_fee = unstake_fee;
  vault.bump = *ctx.bumps.get("vault").unwrap();

  Ok(())
}

pub fn handle_vault_update(
  ctx: Context<UpdateVault>,
  new_authority: Pubkey,
  creator_address: Pubkey,
  payout_interval: u64,
  payout_amount: u64,
  stake_fee: u64,
  unstake_fee: u64,
) -> Result<()> {
  require_keys_eq!(
    ctx.accounts.vault.load()?.authority,
    ctx.accounts.authority.key(),
    CustomError::Unauthorized
  );
  let vault = &mut ctx.accounts.vault.load_mut()?;

  vault.authority = new_authority;
  vault.creator_address = creator_address;
  vault.payout_interval = payout_interval;
  vault.payout_amount = payout_amount;
  vault.stake_fee = stake_fee;
  vault.unstake_fee = unstake_fee;

  Ok(())
}

pub fn handle_fund(ctx: Context<FundSolVault>, amount: u64) -> Result<()> {
  let vault = &mut ctx.accounts.vault.load_mut()?;
  vault.total_amount = vault.total_amount.checked_add(amount).unwrap();
  **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? += amount;
  **ctx.accounts.funder.try_borrow_mut_lamports()? -= amount;

  Ok(())
}

pub fn handle_drain(ctx: Context<DrainSolVault>, amount: u64) -> Result<()> {
  require_keys_eq!(
    ctx.accounts.vault.load()?.authority.key(),
    ctx.accounts.funder.key(),
    CustomError::Unauthorized
  );
  let vault = &mut ctx.accounts.vault.load_mut()?;
  vault.total_amount = vault.total_amount.checked_sub(amount).unwrap();
  **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= amount;
  **ctx.accounts.funder.try_borrow_mut_lamports()? += amount;

  Ok(())
}

pub fn handle_start_payout_schedule(ctx: Context<UpdateVault>) -> Result<()> {
  require_keys_eq!(
    ctx.accounts.vault.load()?.authority.key(),
    ctx.accounts.authority.key(),
    CustomError::Unauthorized
  );
  let vault = &mut ctx.accounts.vault.load_mut()?;

  vault.start_payout_schedule();

  Ok(())
}