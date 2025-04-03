use solana_program::{
    msg, program_error::ProgramError, pubkey::{Pubkey, PUBKEY_BYTES}
};
use crate::errors::FundError;
use borsh::{BorshSerialize, BorshDeserialize};

const BYTE_SIZE_8: usize = 8;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum FundInstruction {

    // 1. Governance Mint Account
    // 2. Vault Account
    // 3. Fund Account
    // 4. System Program
    // 5. Token Program
    // 6. Rent Account
    // 7. [..] Array of Fund Members
    InitFundAccount { 
        number_of_members: u8,
        user_seed: Vec<u8>,
    },

    // 1. Governance Mint Account
    // 2. Vault Account
    // 3. Fund Account
    // 4. System Program
    // 5. Token Program
    // 6. Member's Governance Token Account
    // 7. Member's Wallet
    // 8. User specific pda account
    InitDepositSol {
        amount: u64,
        user_seed: Vec<u8>,
    },


    // 1. Investment proposal (tag -> 0)
    // 2. Addition of new member (tag -> 1)
    // 3. Withdrawal proposal (tag -> 2)
    // 4. Removal of any member (tag -> 3)
    InitProposalAccount { 
        // assets: Vec<Pubkey>, 
        amounts: Vec<u64>, 
        dex: Vec<u8>, 
        deadline: i64,
        user_seed: Vec<u8>,
    },


    // 1. Voter Account
    // 2. Vote Account
    // 3. System Program
    // 4. Fund Account
    // 5. Proposal Account
    // 6. User specified pda
    // 7. Governance token mint
    // 8. Voter's governance token account
    InitVotingAccount { 
        user_seed: Vec<u8>,
        vote: u8,

    },

    Execute { proposal: Pubkey },
}

impl FundInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(FundError::InstructionUnpackError)?;

        Ok(match tag {
            0 => {
                let (num, rest) = Self::unpack_members(rest)?;
                let (user_seed, _rest) = Self::unpack_seed(rest)?;

                Self::InitFundAccount {
                    number_of_members: num,
                    user_seed,
                }
            }
            1 => {
                let (amount, rest) = Self::unpack_amount(rest)?;
                let (user_seed, _rest) = Self::unpack_seed(rest)?;

                Self::InitDepositSol {
                    amount,
                    user_seed
                }
            }

            2 => {
                let (&number_of_assets,rest) = rest.split_first().ok_or(FundError::InstructionUnpackError)?;
                let(amounts,rest) = Self::unpack_amounts(rest,number_of_assets)?;
                let(dex,rest) = Self::unpack_dex(rest, number_of_assets)?;
                let(deadline,rest) = Self::unpack_deadline(rest)?;
                let (user_seed, _rest) = Self::unpack_seed(rest)?;
                Self::InitProposalAccount {
                    amounts,
                    dex,
                    deadline,
                    user_seed,
                }
            }

            3 => {
                let(vote,rest) = Self::unpack_vote(input)?;
                let (user_seed, _rest) = Self::unpack_seed(rest)?;
                Self::InitVotingAccount{
                    vote,
                    user_seed,
                }
            }

            _ => {
                msg!("Instruction cannot be unpacked");
                return Err(FundError::InstructionUnpackError.into());
            }
        })

    }

    fn unpack_members(input: &[u8]) -> Result<(u8, &[u8]), ProgramError> {
        let (&num, rest) = input
            .split_first()
            .ok_or(FundError::InstructionUnpackError)?;

        Ok((num, rest))
    }

    pub fn unpack_amount(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() < BYTE_SIZE_8 {
            msg!("Amount cannot be unpacked");
            return Err(FundError::InstructionUnpackError.into());
        }
        let (amount_bytes, rest) = input.split_at(BYTE_SIZE_8);

        let amount = u64::from_le_bytes(amount_bytes.try_into().expect("Invalid amount length"));

        Ok((amount, rest))
    }

    pub fn unpack_seed(input: &[u8]) -> Result<(Vec<u8>,&[u8]),ProgramError> {

        if input.iter().len() < PUBKEY_BYTES {
            msg!("Invalid seed provided");
            return Err(FundError::InstructionUnpackError.into());
        }

        let mut input_slice = input;
        let mut seed_material:Vec<u8> = Vec::new();

        for _i in 0..PUBKEY_BYTES{
            
            let (seed, rest) = input_slice.split_first().ok_or(FundError::InstructionUnpackError)?;
            seed_material.push(*seed);
            input_slice = rest;
        }

        Ok((seed_material,input_slice))
    }

    pub fn unpack_amounts(input: &[u8], number_of_assets:u8) -> Result<(Vec<u64>, &[u8]), ProgramError> {
        if input.len() < BYTE_SIZE_8*(number_of_assets as usize) {
            msg!("Amount cannot be unpacked");
            return Err(FundError::InstructionUnpackError.into());
        }
        
        let mut amounts = Vec::new();
        let mut input_slice=input;

        for _i in 0..number_of_assets {
            let (amount,rest) = Self::unpack_amount(input_slice)?;
            amounts.push(amount);
            input_slice = rest;
        }
        // let (amount_bytes, rest) = input.split_at(BYTE_SIZE_8);
        // let amount = u64::from_le_bytes(amount_bytes.try_into().expect("Invalid amount length"));

        Ok((amounts, input_slice))
    }

    fn unpack_dex(input: &[u8], number_of_assets:u8) -> Result<(Vec<u8>, &[u8]),ProgramError> {

        if input.len() < number_of_assets as usize {
            msg!("Invalid Dex key provided");
            return Err(FundError::InstructionUnpackError.into());
        }

        let mut dex=Vec::new();
        let mut input_slice=input;
        for _i in 0..number_of_assets {
            let(dex_bytes, rest)= input_slice.split_first().ok_or(FundError::InstructionUnpackError)?;
            dex.push(*dex_bytes);
            input_slice=rest;
        }

        // let (dex_bytes,rest) = input.split_at(PUBKEY_BYTES);
        // let dex = Pubkey::try_from_slice(dex_bytes).map_err(|_| ProgramError::InvalidAccountData)?;

        Ok((dex,input_slice))
    }

    fn unpack_deadline (input: &[u8]) -> Result<(i64, &[u8]), ProgramError> {

        if input.len() < BYTE_SIZE_8 {
            msg!("Invalid deadline provided");
            return Err(FundError::InstructionUnpackError.into());
        }

        let(deadline_bytes, rest) = input.split_at(BYTE_SIZE_8);

        let deadline=i64::from_le_bytes(deadline_bytes.try_into().expect("Invalid deadline provided"));

        Ok((deadline,rest))
    }

    fn unpack_vote (input: &[u8]) ->Result<(u8,&[u8]), ProgramError> {

        if input.len() < BYTE_SIZE_8 {
            msg!("Invalid Voting");
            return Err(FundError::InstructionUnpackError.into());
        }

        let (vote,rest) = input.split_first().ok_or(FundError::InstructionUnpackError)?;

        Ok((*vote,rest))
    }

}