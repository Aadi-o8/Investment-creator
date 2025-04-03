use solana_program::pubkey::Pubkey;
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct FundAccount {
    pub members: Vec<Pubkey>,
    pub total_deposit: u64,
    pub governance_mint: Pubkey,
    pub vault: Pubkey,
    pub is_initialized: bool,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct UserspecificAccount {
    pub user:Pubkey,
    pub deposit:u64,
    pub is_active:bool,
    pub governance_token_balance:u64,
    pub governance_token_account:Pubkey,
    pub number_of_proposals:u64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ProposalAccount {
    pub proposer: Pubkey,
    pub from_asset: Vec<Pubkey>,
    pub to_asset: Vec<Pubkey>,
    pub amount: Vec<u64>,
    pub dex: Vec<u8>,
    pub votes_yes: u64,
    pub votes_no: u64,
    pub deadline: i64,
    pub executed: bool,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct VotingAccount {
    pub voter: Pubkey,
    pub vote: bool,
    pub voting_power: u64,
}