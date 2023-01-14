use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
  program::{invoke, invoke_signed}
};
use mpl_token_metadata::instruction::{freeze_delegated_account, thaw_delegated_account};

use crate::errors::*;
use crate::ins::*;
/*
* Stake:: Stake Instruction - Stake the user's NFT.
*/
pub fn handle_stake(ctx: Context<Stake>) -> Result<()> {
  let vault = &mut ctx.accounts.vault.load_mut()?;
  let user = &mut ctx.accounts.user;
  let token_mint = ctx.accounts.token_mint.key();

  let mut is_max_staked = false;
  if user.mint_staked_count >= 200 {
    is_max_staked = true
  }
  require_eq!(is_max_staked, false, CustomError::MaxStaked);

  // Load the NFT metadata
  let metadata = spl_token_metadata::state::Metadata::from_account_info(&ctx.accounts.nft_metadata_account)?;
  let creators = metadata.data.creators.unwrap();
  let mut creator_found = false;
  for creator in creators {
    if creator.address.key() == vault.creator_address {
      creator_found = true;
    }
  }

  // NFT must be created by whitelist owner.
  require_eq!(creator_found, true, CustomError::WrongNFT);
  
  if vault.stake_fee > 0 {
    invoke(
      &anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.staker.key,
        &ctx.accounts.vault.key(),
        vault.stake_fee,
      ),
      &[
        ctx.accounts.staker.to_account_info().clone(),
        ctx.accounts.vault.to_account_info().clone(),
        ctx.accounts.system_program.to_account_info().clone(),
      ],
    )?;
    vault.total_earned = vault.total_earned.checked_add(vault.stake_fee).unwrap();
  }
  
  vault.update_payout_round();
  user.add_item(vault, token_mint)?;

  let cpi_context = CpiContext::new(
    ctx.accounts.token_program.to_account_info(),
    anchor_spl::token::Approve {
        to: ctx.accounts.staker_ata.to_account_info().clone(),
        delegate: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.staker.to_account_info()
    }
  );

  anchor_spl::token::approve(cpi_context, 1)?;
  
  // Get the NFT from the Vault,
  let token_vault_bump = vault.bump;
  let seeds = &[
    b"vault".as_ref(),
    &[token_vault_bump],
  ];

  invoke_signed(
      &freeze_delegated_account(
          ctx.accounts.token_metadata_program.key(),
          ctx.accounts.vault.key(),
          ctx.accounts.staker_ata.key(),
          ctx.accounts.edition.key(),
          ctx.accounts.token_mint.key(),
      ),
      &[
          ctx.accounts.vault.to_account_info(),
          ctx.accounts.staker_ata.to_account_info(),
          ctx.accounts.edition.to_account_info(),
          ctx.accounts.token_mint.to_account_info()
      ],
      &[seeds]
  )?;

  Ok(())
}

/*
* Unstake:: Untake Instruction - Unstake the user's NFT.
*/
pub fn handle_unstake(ctx: Context<Unstake>) -> Result<()> {
  let vault = &mut ctx.accounts.vault.load_mut()?;
  let user = &mut ctx.accounts.user;
  let token_mint = ctx.accounts.token_mint.key();
  
  // Staker should own staker account
  require_keys_eq!(
    ctx.accounts.staker.key(),
    user.key(),
    CustomError::KeyMismatch
  );

  // If the staker key is not the same as the signer key,
  // then the signer account should match authority key.
  if ctx.accounts.staker.key() != ctx.accounts.signer.key() {
    require_keys_eq!(
      ctx.accounts.signer.key(),
      vault.authority.key(),
      CustomError::Unauthorized
    );
  }

  if vault.unstake_fee > 0 {
    invoke(
      &anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.staker.key,
        &ctx.accounts.vault.key(),
        vault.unstake_fee,
      ),
      &[
        ctx.accounts.staker.to_account_info().clone(),
        ctx.accounts.vault.to_account_info().clone(),
        ctx.accounts.system_program.to_account_info().clone(),
      ],
    )?;
    vault.total_earned = vault.total_earned.checked_add(vault.unstake_fee).unwrap();

  }
  
  vault.update_payout_round();
  user.remove_item(vault, token_mint)?;

  // Get the NFT from the Vault,
  let token_vault_bump = vault.bump;

  let seeds = &[
    b"vault".as_ref(),
    &[token_vault_bump],
  ];
  invoke_signed(
    &thaw_delegated_account(
        ctx.accounts.token_metadata_program.key(),
        ctx.accounts.vault.key(),
        ctx.accounts.staker_ata.key(),
        ctx.accounts.edition.key(),
        ctx.accounts.token_mint.key(),
    ),
    &[
        ctx.accounts.vault.to_account_info(),
        ctx.accounts.staker_ata.to_account_info(),
        ctx.accounts.edition.to_account_info(),
        ctx.accounts.token_mint.to_account_info()
    ],
    &[seeds]
  )?;


  if ctx.accounts.staker.key() == ctx.accounts.signer.key() {
    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        anchor_spl::token::Revoke {
            source: ctx.accounts.staker_ata.to_account_info(),
            authority: ctx.accounts.staker.to_account_info()
        }
    );

    anchor_spl::token::revoke(cpi_context)?;
  }

  Ok(())
}
