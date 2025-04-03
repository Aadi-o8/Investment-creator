// use core::error;

use crate::{
    errors::FundError,
    instruction::FundInstruction,
    state::{FundAccount, ProposalAccount, UserspecificAccount, VotingAccount},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::state::Account as TokenAccount;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let instruction = FundInstruction::unpack(data)?;
    match instruction {
        FundInstruction::InitFundAccount {
            number_of_members,
            user_seed,
        } => {
            msg!("Instruction: Init Fund Account");
            process_init_fund_account(program_id, accounts, number_of_members, user_seed)
        }

        FundInstruction::InitDepositSol { amount, user_seed } => {
            msg!("Instruction: Init Deposit");
            process_init_deposit_sol(program_id, accounts, amount, user_seed)
        }

        FundInstruction::InitProposalAccount {
            amounts,
            dex,
            deadline,
            user_seed,
        } => {
            msg!("Instruction: Init Proposal");
            process_init_proposal_creation(program_id, accounts, amounts, dex, deadline, user_seed)
        }

        FundInstruction::InitVotingAccount { user_seed, vote } => {
            msg!("Instruction: Init Voting Account");
            process_init_voting(program_id, accounts, vote, user_seed)
        }

        _ => Err(FundError::InvalidInstruction.into()),
    }
}

fn process_init_fund_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    number_of_members: u8,
    user_seeds: Vec<u8>,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let governance_mint_info = next_account_info(accounts_iter)?;
    let vault_account_info = next_account_info(accounts_iter)?;
    let system_program_info = next_account_info(accounts_iter)?;
    let token_program_info = next_account_info(accounts_iter)?;
    let fund_account_info = next_account_info(accounts_iter)?;
    let rent_account_info = next_account_info(accounts_iter)?;

    let members: Vec<&AccountInfo> = accounts_iter.take(number_of_members as usize).collect();

    if members.len() != number_of_members as usize || !members.iter().all(|m| m.is_signer) {
        return Err(FundError::MissingRequiredSignature.into());
    }

    let member_pubkeys: Vec<Pubkey> = members.iter().map(|m| *m.key).collect();
    // member_pubkeys.sort();

    // seeds.extend_from_slice(b"fund");

    // for pubkey in member_pubkeys {
    //     seeds.extend_from_slice(pubkey.as_ref());
    // }

    let seeds = user_seeds;
    let seeds_array: &[u8] = &seeds;

    let (fund_pda, _fund_bump) = Pubkey::find_program_address(&[seeds_array, b"fund"], program_id);
    let (vault_pda, _vault_bump) =
        Pubkey::find_program_address(&[b"vault", fund_pda.as_ref()], program_id);
    let (_rent_pda, rent_bump) = Pubkey::find_program_address(&[b"rent_69"], program_id);
    if *fund_account_info.key != fund_pda || *vault_account_info.key != vault_pda {
        return Err(FundError::InvalidAccountData.into());
    }

    let rent = Rent::get()?;
    let fund_space = 73 + 32 * (number_of_members as i32);
    let vault_space = 165;
    let mint_space = 82;
    let total_rent = rent.minimum_balance(fund_space as usize)
        + rent.minimum_balance(vault_space)
        + rent.minimum_balance(mint_space);
    let rent_per_member = total_rent / u64::from(number_of_members);

    for member in &members {
        invoke(
            &system_instruction::transfer(member.key, rent_account_info.key, rent_per_member),
            &[
                (*member).clone(),
                fund_account_info.clone(),
                system_program_info.clone(),
            ],
        )?;
    }

    invoke_signed(
        &system_instruction::create_account(
            rent_account_info.key,
            fund_account_info.key,
            rent.minimum_balance(fund_space as usize),
            fund_space as u64,
            program_id,
        ),
        &[
            rent_account_info.clone(),
            fund_account_info.clone(),
            system_program_info.clone(),
        ],
        &[&[b"rent_69", &[rent_bump]]],
    )?;

    invoke_signed(
        &system_instruction::create_account(
            rent_account_info.key,
            vault_account_info.key,
            rent.minimum_balance(vault_space),
            vault_space as u64,
            token_program_info.key,
        ),
        &[
            rent_account_info.clone(),
            vault_account_info.clone(),
            system_program_info.clone(),
        ],
        &[&[b"rent_69", &[rent_bump]]],
    )?;

    invoke_signed(
        &system_instruction::create_account(
            rent_account_info.key,
            governance_mint_info.key,
            rent.minimum_balance(mint_space),
            mint_space as u64,
            token_program_info.key,
        ),
        &[
            rent_account_info.clone(),
            governance_mint_info.clone(),
            system_program_info.clone(),
        ],
        &[&[b"rent_69", &[rent_bump]]],
    )?;
    invoke(
        &spl_token::instruction::initialize_mint(
            token_program_info.key,
            governance_mint_info.key,
            fund_account_info.key,
            None,
            6,
        )?,
        &[governance_mint_info.clone(), token_program_info.clone()],
    )?;

    let fund_data = FundAccount {
        members: member_pubkeys,
        total_deposit: 0,
        governance_mint: *governance_mint_info.key,
        vault: *vault_account_info.key,
        is_initialized: true,
    };
    fund_data.serialize(&mut &mut fund_account_info.data.borrow_mut()[..])?;

    Ok(())
}

fn process_init_deposit_sol(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
    user_seeds: Vec<u8>,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let governance_mint_info = next_account_info(accounts_iter)?;
    let vault_account_info = next_account_info(accounts_iter)?;
    let fund_account_info = next_account_info(accounts_iter)?;
    let system_program_info = next_account_info(accounts_iter)?;
    let token_program_info = next_account_info(accounts_iter)?;
    let governance_token_account = next_account_info(accounts_iter)?;
    let member_account = next_account_info(accounts_iter)?;
    let user_specific_pda_info = next_account_info(accounts_iter)?;

    if !member_account.is_signer {
        msg!("Required signer not found");
        return Err(FundError::MissingRequiredSignature.into());
    }

    let fund_data = FundAccount::try_from_slice(&fund_account_info.data.borrow())?;
    // let member_pubkeys = fund_data.members.clone();

    let seeds = &user_seeds;
    let seeds_array: &[u8] = &seeds;

    let (fund_pda, fund_bump) = Pubkey::find_program_address(&[b"fund", seeds_array], program_id);
    let (vault_pda, _vault_bump) =
        Pubkey::find_program_address(&[b"vault", fund_pda.as_ref()], program_id);
    let (user_pda, _user_bump) = Pubkey::find_program_address(
        &[b"user_pda", fund_pda.as_ref(), member_account.key.as_ref()],
        program_id,
    );

    if *fund_account_info.key != fund_pda
        || *vault_account_info.key != vault_pda
        || *user_specific_pda_info.key != user_pda
    {
        msg!("Either wrong Fund Account given or either wrong Vault Account give");
        return Err(FundError::InvalidAccountData.into());
    }

    if fund_data.governance_mint != *governance_mint_info.key {
        msg!("Wrong Governance mint key");
        return Err(FundError::InvalidAccountData.into());
    }

    if governance_token_account.data_is_empty() {
        invoke(
            &spl_associated_token_account::instruction::create_associated_token_account(
                member_account.key,
                member_account.key,
                governance_mint_info.key,
                token_program_info.key,
            ),
            &[
                member_account.clone(),
                governance_mint_info.clone(),
                token_program_info.clone(),
            ],
        )?;

        invoke(
            &system_instruction::transfer(member_account.key, vault_account_info.key, amount),
            &[
                member_account.clone(),
                vault_account_info.clone(),
                system_program_info.clone(),
            ],
        )?;

        invoke_signed(
            &spl_token::instruction::mint_to(
                token_program_info.key,
                governance_mint_info.key,
                governance_token_account.key,
                fund_account_info.key,
                &[],
                amount,
            )?,
            &[
                token_program_info.clone(),
                governance_mint_info.clone(),
                governance_token_account.clone(),
                fund_account_info.clone(),
            ],
            &[&[b"fund_pda", &seeds_array, &[fund_bump]]],
        )?;
    }

    let mut fund_data = FundAccount::try_from_slice(&fund_account_info.data.borrow())?;
    fund_data.total_deposit += amount;
    fund_data.serialize(&mut &mut fund_account_info.data.borrow_mut()[..])?;

    let mut user_data = UserspecificAccount::try_from_slice(&user_specific_pda_info.data.borrow())?;
    user_data.deposit += amount;
    user_data.governance_token_balance += amount;
    user_data.is_active = true;
    user_data.user = *user_specific_pda_info.key;
    user_data.governance_token_account = *governance_token_account.key;
    user_data.serialize(&mut &mut user_specific_pda_info.data.borrow_mut()[..])?;

    Ok(())
}

fn process_init_proposal_creation(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amounts: Vec<u64>,
    dex: Vec<u8>,
    deadline: i64,
    user_seeds: Vec<u8>,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let proposer_account_info = next_account_info(account_iter)?;
    let proposal_account_info = next_account_info(account_iter)?;
    let fund_account_info = next_account_info(account_iter)?;
    let user_specific_pda_info = next_account_info(account_iter)?;
    // let vault_account_info = next_account_info(account_iter)?;
    let system_program_info = next_account_info(account_iter)?;

    if !proposer_account_info.is_signer {
        msg!("Invalid Account info.");
        return Err(FundError::InvalidAccountData.into());
    }

    let seeds = &user_seeds;
    let seeds_array: &[u8] = &seeds;

    let (fund_pda, _fund_bump) =
        Pubkey::find_program_address(&[b"fund_pda", seeds_array], program_id);
    let (proposal_pda, _proposer_bump) = Pubkey::find_program_address(
        &[
            b"proposer_pda",
            fund_account_info.key.as_ref(),
            proposer_account_info.key.as_ref(),
        ],
        program_id,
    );
    let mut user_data =
        UserspecificAccount::try_from_slice(&user_specific_pda_info.data.borrow_mut())?;
    let (user_pda, _user_bump) = Pubkey::find_program_address(
        &[
            b"user_69",
            fund_account_info.key.as_ref(),
            proposer_account_info.key.as_ref(),
        ],
        program_id,
    );

    if *fund_account_info.key != fund_pda
        || *proposal_account_info.key != proposal_pda
        || *user_specific_pda_info.key != user_pda
    {
        msg!("Invalid Account info");
        return Err(FundError::InvalidAccountData.into());
    }

    let rent = Rent::get()?;
    let proposal_space = 43 + amounts.len() * 73;
    let total_rent = rent.minimum_balance(proposal_space);

    invoke(
        &system_instruction::create_account(
            proposer_account_info.key,
            proposal_account_info.key,
            total_rent,
            proposal_space as u64,
            program_id,
        ),
        &[
            proposer_account_info.clone(),
            proposal_account_info.clone(),
            system_program_info.clone(),
        ],
    )?;

    let from_assets_info: Vec<&AccountInfo> = account_iter.take(amounts.len()).collect();

    if from_assets_info.len() != amounts.len() {
        return Err(FundError::InvalidAccountData)?;
    }

    let from_assets_mint: Vec<Pubkey> = from_assets_info.iter().map(|m| *m.key).collect();

    let to_assets_info: Vec<&AccountInfo> = account_iter.take(amounts.len()).collect();

    if to_assets_info.len() != amounts.len() {
        return Err(FundError::InvalidAccountData.into());
    }

    let to_assets_mint: Vec<Pubkey> = to_assets_info.iter().map(|m| *m.key).collect();

    let proposal_data = ProposalAccount {
        proposer: *proposer_account_info.key,
        from_asset: from_assets_mint,
        to_asset: to_assets_mint,
        amount: amounts,
        dex,
        votes_yes: 0,
        votes_no: 0,
        deadline,
        executed: false,
    };

    proposal_data.serialize(&mut &mut proposal_account_info.data.borrow_mut()[..])?;

    user_data.number_of_proposals += 1;

    Ok(())
}

fn process_init_voting(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    vote: u8,
    user_seed: Vec<u8>,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let voter_account_info = next_account_info(account_iter)?;
    let vote_account_info = next_account_info(account_iter)?;
    let system_program_info = next_account_info(account_iter)?;
    let fund_account_info = next_account_info(account_iter)?;
    let proposal_account_info = next_account_info(account_iter)?;
    let user_specific_pda = next_account_info(account_iter)?;
    let governance_mint_account_info = next_account_info(account_iter)?;
    let governance_token_account = next_account_info(account_iter)?;

    let current_time = Clock::get()?.unix_timestamp;

    if !voter_account_info.is_signer {
        msg!("Invalid signer");
        return Err(FundError::InvalidAccountData)?;
    }

    let fund_data = FundAccount::try_from_slice(&fund_account_info.data.borrow())?;
    let proposal_data = ProposalAccount::try_from_slice(&proposal_account_info.data.borrow())?;
    let user_data = UserspecificAccount::try_from_slice(&user_specific_pda.data.borrow())?;

    if proposal_data.deadline < current_time {
        msg!("Sorry, you passed after the deadline");
        return Err(FundError::InvalidInstruction.into());
    }

    let (proposal_pda, _proposa_bump) = Pubkey::find_program_address(
        &[
            b"proposal_pda",
            fund_account_info.key.as_ref(),
            voter_account_info.key.as_ref(),
        ],
        program_id,
    );
    let (fund_pda, _fund_bump) =
        Pubkey::find_program_address(&[b"fund_pda", &user_seed], program_id);
    let (user_pda, _user_bump) = Pubkey::find_program_address(
        &[
            b"user_pda",
            fund_account_info.key.as_ref(),
            voter_account_info.key.as_ref(),
        ],
        program_id,
    );
    let token_account = spl_associated_token_account::get_associated_token_address(
        voter_account_info.key,
        governance_mint_account_info.key,
    );

    if *governance_mint_account_info.key != fund_data.governance_mint
        || *proposal_account_info.key != proposal_pda
        || *fund_account_info.key != fund_pda
        || *user_specific_pda.key != user_pda
        || token_account != user_data.governance_token_account
    {
        return Err(FundError::InvalidAccountData)?;
    }

    let rent = Rent::get()?;
    let vote_space = 33 as usize;
    let total_rent = rent.minimum_balance(vote_space);

    if vote_account_info.data_is_empty() {
        invoke(
            &system_instruction::create_account(
                voter_account_info.key,
                vote_account_info.key,
                total_rent,
                vote_space as u64,
                system_program_info.key,
            ),
            &[
                voter_account_info.clone(),
                vote_account_info.clone(),
                system_program_info.clone(),
            ],
        )?;

        let token_account_data = TokenAccount::unpack(&governance_token_account.data.borrow())?;
        let mut voter_data = VotingAccount::try_from_slice(&vote_account_info.data.borrow())?;

        let voting_power = token_account_data.amount;

        voter_data.voting_power = voting_power;

        if vote as u64 != 1 {
            voter_data.vote = false;
        }

        if vote as u64 == 1 {
            voter_data.vote = true;
        }
    } else {
        msg!("Already voted!");
        return Err(FundError::InvalidInstruction.into());
    }

    Ok(())
}
