use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{state::*};

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds =[
            b"vault".as_ref(),
        ],
        bump
    )]
    pub vault: AccountLoader<'info, Vault>,
}

#[derive(Accounts)]
pub struct UpdateVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"vault".as_ref(),
        ],
        bump = vault.load()?.bump
    )]
    pub vault: AccountLoader<'info, Vault>,
}

#[derive(Accounts)]
pub struct FundSolVault<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"vault".as_ref(),
        ],
        bump = vault.load()?.bump
    )]
    pub vault: AccountLoader<'info, Vault>,
}

#[derive(Accounts)]
pub struct DrainSolVault<'info> 
{
    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"vault".as_ref(),
        ],
        bump = vault.load()?.bump
    )]
    pub vault: AccountLoader<'info, Vault>,
}

#[derive(Accounts)]
pub struct CreateStakeAccount<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        seeds = [
            b"user".as_ref(),
            user.key().as_ref()
        ],
        bump,
        space = User::LEN + 8
    )]
    pub user: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,
    
    #[account(mut)]
    pub staker_account: AccountLoader<'info, User>,

    #[account(
        mut,
        seeds = [
            b"vault".as_ref(),
            vault.name.as_ref(),
        ],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,


    // The Mint Account for the NFT.
    pub token_mint: Account<'info, Mint>,

    // The Token Account holding the NFT.
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = staker,
    )]
    pub staker_ata: Box<Account<'info, TokenAccount>>,

    // // The metadata account of the NFT.
    /// CHECK:
    pub nft_metadata_account: AccountInfo<'info>,

    /// CHECK:
    pub edition: AccountInfo<'info>,
    // Accounts Required for init instruction
    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
    // the token metadata program
    /// CHECK:
    #[account(constraint = token_metadata_program.key == &metaplex_token_metadata::ID)]
    pub token_metadata_program: AccountInfo<'info>,
    
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub staker: SystemAccount<'info>,

    #[account(mut)]
    pub staker_account: AccountLoader<'info, User>,

    #[account(
        mut,
        seeds = [
            b"vault".as_ref(),
            vault.name.as_ref(),
        ],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut, address = vault.community_wallet)]
    pub community_wallet: SystemAccount<'info>,

    pub token_mint: Account<'info, Mint>,

    pub booster_mint: Account<'info, Mint>,

    // The Token Account holding the NFT.
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = staker,
    )]
    pub staker_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = booster_mint,
        associated_token::authority = staker,
    )]
    pub booster_ata: Box<Account<'info, TokenAccount>>,

    /// CHECK:
    pub edition: AccountInfo<'info>,

    /// CHECK:
    pub booster_edition: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
    // the token metadata program
    /// CHECK:
    #[account(constraint = token_metadata_program.key == &metaplex_token_metadata::ID)]
    pub token_metadata_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Boost<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,
    
    #[account(mut)]
    pub staker_account: AccountLoader<'info, User>,

    #[account(
        mut,
        seeds = [
            b"vault".as_ref(),
            vault.name.as_ref(),
        ],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,


    // The Mint Account for the NFT.
    pub token_mint: Account<'info, Mint>,

    pub booster_mint: Account<'info, Mint>,

    // The Token Account holding the NFT.
    #[account(
        mut,
        associated_token::mint = booster_mint,
        associated_token::authority = staker,
    )]
    pub booster_ata: Box<Account<'info, TokenAccount>>,

    // // The metadata account of the NFT.
    /// CHECK:
    pub nft_metadata_account: AccountInfo<'info>,

    /// CHECK:
    pub edition: AccountInfo<'info>,
    // Accounts Required for init instruction
    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
    // the token metadata program
    /// CHECK:
    #[account(constraint = token_metadata_program.key == &metaplex_token_metadata::ID)]
    pub token_metadata_program: AccountInfo<'info>,
    
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct UnBoost<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,
    
    #[account(mut)]
    pub staker_account: AccountLoader<'info, User>,

    #[account(
        mut,
        seeds = [
            b"vault".as_ref(),
            vault.name.as_ref(),
        ],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,


    // The Mint Account for the NFT.
    pub token_mint: Account<'info, Mint>,

    pub booster_mint: Account<'info, Mint>,

    // The Token Account holding the NFT.
    #[account(
        mut,
        associated_token::mint = booster_mint,
        associated_token::authority = staker,
    )]
    pub booster_ata: Box<Account<'info, TokenAccount>>,

    /// CHECK:
    pub edition: AccountInfo<'info>,
    // Accounts Required for init instruction
    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
    // the token metadata program
    /// CHECK:
    #[account(constraint = token_metadata_program.key == &metaplex_token_metadata::ID)]
    pub token_metadata_program: AccountInfo<'info>,
    
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub staker: SystemAccount<'info>,

    #[account(mut)]
    pub staker_account: AccountLoader<'info, User>,

    #[account(
        mut,
        seeds = [
            b"vault".as_ref(),
            vault.name.as_ref(),
        ],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,

    pub reward_token_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = staker,
        associated_token::mint = reward_token_mint,
        associated_token::authority = staker,
    )]
    pub staker_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = reward_token_mint,
        associated_token::authority = vault,
    )]
    pub reward_token_vault_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ClosePda<'info> {
    #[account(mut, address = "3qWq2ehELrVJrTg2JKKERm67cN6vYjm1EyhCEzfQ6jMd".parse::<Pubkey>().unwrap())]
    pub signer: Signer<'info>,

    /// CHECK:
    #[account(mut)]
    pub pda: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}