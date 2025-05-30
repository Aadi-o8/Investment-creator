use solana_program::pubkey::Pubkey;
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct FundAccount {
    pub name: [u8; 32],
    pub creator: Pubkey,
    pub members: u64,
    pub total_deposit: u64,
    pub governance_mint: Pubkey,
    pub vault: Pubkey,
    pub is_initialized: bool,
    pub created_at: i64,
    pub is_private: u8,
    // pub dex_program_ids: Vec<(u8, Pubkey)>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct VaultAccount {
    pub fund: Pubkey,
    pub last_deposit_time: i64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UserAccount {
    pub user: Pubkey,
    pub funds: Vec<Pubkey>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UserSpecificAccount {
    pub pubkey: Pubkey,
    pub fund: Pubkey,
    pub deposit: u64,
    pub governance_token_balance: u64,
    pub is_active: bool,
    pub num_proposals: u8,
    pub join_time: i64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct InvestmentProposalAccount {
    pub fund: Pubkey,
    pub proposer: Pubkey,
    pub from_assets: Vec<Pubkey>,
    pub to_assets: Vec<Pubkey>,
    pub amounts: Vec<u64>,
    // pub dex_tags: Vec<u8>,
    pub votes_yes: u64,
    pub votes_no: u64,
    pub deadline: i64,
    pub executed: bool,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct VoteAccount {
    pub voter: Pubkey,
    pub vote: u8,
}
