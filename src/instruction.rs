use solana_program::{
    program_error::ProgramError,
    pubkey::{Pubkey, PUBKEY_BYTES},
    msg
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
        amount: u64
    },
    InitProposalAccount { asset: Pubkey, amount: u64, dex: Pubkey, deadline: i64 },
    Vote { proposal: Pubkey, yes: bool, amount: u64 },
    Execute { proposal: Pubkey },
}

impl FundInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(FundError::InstructionUnpackError)?;

        Ok(match tag {
            0 => {
                let (num, _rest) = Self::unpack_members(rest)?;

                Self::InitFundAccount {
                    number_of_members: num,
                }
            }
            1 => {
                let (amount, _rest) = Self::unpack_amount(rest)?;

                Self::InitDepositSol {
                    amount,
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

    fn unpack_amount(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() < BYTE_SIZE_8 {
            msg!("Amount cannot be unpacked");
            return Err(FundError::InstructionUnpackError.into());
        }
        let (amount_bytes, rest) = input.split_at(BYTE_SIZE_8);

        let amount = u64::from_le_bytes(amount_bytes.try_into().expect("Invalid amount length"));

        Ok((amount, rest))
    }

}