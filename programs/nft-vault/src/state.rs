use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;

#[account(zero_copy)]
pub struct Vault {
    pub authority: Pubkey,
    pub creator_address: Pubkey,
    pub total_earned: u64,
    pub payout_schedule_started_time: u64,
    pub payout_round: u32,
    pub payout_interval: u64,
    pub payout_amount: u64,
    pub total_staked_count: u32,
    pub total_eligible_count: u32,
    pub total_amount: u64,
    pub stake_fee: u64,
    pub unstake_fee: u64,
    pub total_items_count: u32,
    pub bump: u8,
    pub total_balance_items: [BalanceItem; TOTAL_COLLECTION_COUNT],
}

impl Vault {
    pub const LEN: usize = std::mem::size_of::<Vault>();

    fn start_payout_schedule(&mut self) {
        let now: u64 = clock::Clock::get().unwrap().unix_timestamp.try_into().unwrap();
        self.payout_schedule_started_time = now;
        self.payout_round = 1;
    }

    fn update_payout_round(&mut self) {
        let now: u64 = clock::Clock::get().unwrap().unix_timestamp.try_into().unwrap();
        if self.payout_schedule_started_time == 0 || self.total_eligible_count == 0 ||
            now < self.payout_schedule_started_time.checked_add(
                self.payout_interval.checked_mul(self.payout_round as u64).unwrap()
            ).unwrap() {
            return;
        }
        let prev_round = self.payout_round;
        self.payout_round = now.checked_sub(payout_schedule_started_time).unwrap().checked_div(payout_interval).unwrap().checked_add(1).unwrap();
        let added_round = self.payout_round.checked_sub(prev_round);
        let earned = self.payout_amount.checked_div(self.total_eligible_count).unwrap().checked_mul(added_round).unwrap();
        for i in 0..TOTAL_COLLECTION_COUNT {
            let mut balance_item = self.total_balance_items[i];
            if balance_item.state == 1 {
                balance_item.state = 2;
                balance_item.balance = balance_item.balance.checked_add(earned).unwrap();
                self.total_eligible_count = self.eligible_count.checked_add(1).unwrap();
                self.total_balance_items[i] = balance_item;
            }
        }
    }

    fn add_balance_item(&mut self) -> usize {
        for i in 0..TOTAL_COLLECTION_COUNT {
            let balance_item = self.total_balance_items[i];
            if balance_item.state == 0 {
                balance_item.state = 1;
                self.total_staked_count = self.total_staked_count.checked_add(1).unwrap();
                if self.payout_schedule_started_time == 0 {
                    balance_item.state = 2;
                    self.total_eligible_count = self.total_eligible_count.checked_add(1).unwrap();
                }
                return i
            }
        }
        TOTAL_COLLECTION_COUNT
    }

    fn remove_balance_item(&mut self, index: usize) {
        self.total_balance_items[index].state = 0;
        self.total_eligible_count = self.total_eligible_count.checked_sub(1).unwrap();
        self.total_staked_count = self.total_staked_count.checked_sub(1).unwrap();
    }
}

impl Default for Vault {
    fn default() -> Vault {
        Vault {
            authority: Pubkey::default(),
            creator_address: Pubkey::default(),
            total_earned: 0,
            payout_schedule_started_time: 0,
            payout_round: 0,
            payout_interval: 0,
            payout_amount: 0,
            total_staked_count: 0,
            total_eligible_count: 0,
            total_amount: 0,
            stake_fee: 0,
            unstake_fee: 0,
            total_items_count: 0
            bump: 0,
            total_balance_items: [BalanceItem::default(); TOTAL_COLLECTION_COUNT],
        }
    }
}

#[zero_copy]
#[derive(Debug, PartialEq)]
pub struct BalanceItem {
    pub balance: u64,
    pub state: u8,
}

impl default for BalanceItem {
    fn default() -> BalanceItem {
        BalanceItem {
            balance: 0,
            state: 0,
        }
    }
}


#[account]
pub struct User {
    pub key: Pubkey,
    pub eligible_count: u32,
    pub mint_staked_count: u32,
    pub bump: u8,
    pub staked_items: Vec<StakedNft>, 
}

impl User {
    pub const LEN: usize = std::mem::size_of::<User>() + StakedNft::LEN * MAX_NFT_PER_USER;

    fn add_item(&mut self, vault: &RefMut<Vault>, mint: Pubkey) -> Result<()> {
        if self.staked_items.iter().any(|x| x.mint == mint) return Error(CustomError::AlreadyStaked.into());
        let index = vault.add_balance_item();
        if vault.staked_items[index].state == 2 {
            self.eligible_count = self.eligible_count.checked_add(1).unwrap();
        }
        self.mint_staked_count = self.mint_staked_count.checked_add(1).unwrap();

        Ok(())
    }

    fn remove_item(&mut self, vault: &RefMut<Vault>, mint: Pubkey) -> Result<()> {
        let index = self.staked_items.iter().position(|x| x.mint == mint).unwrap();
        if vault.staked_items[index].state == 2 {
            self.eligible_count = self.eligible_count.checked_sub(1).unwrap();
        }
        self.mint_staked_count = self.mint_staked_count.checked_sub(1).unwrap();
        vault.remove_balance_item(index);

        Ok(())
    }
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct StakedNft {
    pub index: usize,
    pub mint: Pubkey,
}

impl StakedNft {
    pub const LEN: usize = std::mem::size_of::<StakedNft>();
}
