use anchor_lang::prelude::*;

use crate::ins::*;


pub fn handle_create_stake_account(ctx: Context<CreateStakeAccount>) -> Result<()> {
  let user = &mut ctx.accounts.user;
  user.key = ctx.accounts.creator.key();
  user.mint_staked_count = 0;
  user.eligible_count = 0;
  user.staked_items = vec![];
  user.bump = *ctx.bumps.get("user").unwrap();
  Ok(())
}

pub fn handle_claim_rewards(ctx: Context<Claim>) -> Result<()> {
  let vault =  &mut ctx.accounts.vault.load_mut()?;
  let user = &mut ctx.accounts.user;

  vault.update_payout_round();
  let staker_earned_amount = user.claim(vault);

  **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= staker_earned_amount;
  **ctx.accounts.staker.try_borrow_mut_lamports()? += staker_earned_amount;  

  Ok(())
}

