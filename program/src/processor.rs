//! Resolve instruction and execute

use crate::{
    account_state::ProgramAccountState, error::DataVersionError, instruction::ProgramInstruction,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
};

/// Checks each tracking account to confirm it is owned by our program
/// This function assumes that the program account is always the last
/// in the array
fn check_account_ownership(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    // Accounts must be owned by the program.
    for account in accounts.iter().take(accounts.len() - 1) {
        if account.owner != program_id {
            msg!(
                "Fail: The tracking account owner is {} and it should be {}.",
                account.owner,
                program_id
            );
            return Err(ProgramError::IncorrectProgramId);
        }
    }
    Ok(())
}

/// Initialize the programs account, which is the first in accounts
fn initialize_account(accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Initialize account");
    let account_info_iter = &mut accounts.iter();
    let program_account = next_account_info(account_info_iter)?;
    let mut account_data = program_account.data.borrow_mut();

    // Just using unpack will check to see if initialized and will
    // fail if not
    let mut account_state = ProgramAccountState::unpack_unchecked(&account_data)?;
    // Where this is a logic error in trying to initialize the same
    // account more than once
    if account_state.is_initialized() {
        return Err(DataVersionError::AlreadyInitializedState.into());
    } else {
        account_state.set_initialized();
        account_state.get_content_mut().somekey = program_account.key.clone();
    }

    ProgramAccountState::pack(account_state, &mut account_data).unwrap();
    Ok(())
}
/// Main processing entry point dispatches to specific
/// instruction handlers
pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Received process request");
    // Check the account for program relationship
    if let Err(error) = check_account_ownership(program_id, accounts) {
        return Err(error);
    }
    // Unpack the inbound data, mapping instruction to appropriate structure
    let instruction = ProgramInstruction::unpack(instruction_data)?;
    match instruction {
        ProgramInstruction::InitializeAccount => initialize_account(accounts),
        //     ProgramInstruction::MintToAccount { key, value } => {
        //         mint_keypair_to_account(accounts, key, value)
        //     }
        //     ProgramInstruction::TransferBetweenAccounts { key } => {
        //         transfer_keypair_to_account(accounts, key)
        //     }
        //     ProgramInstruction::BurnFromAccount { key } => burn_keypair_from_account(accounts, key),
        //     ProgramInstruction::MintToAccountWithFee { key, value } => {
        //         mint_keypair_to_account_with_fee(accounts, key, value)
        //     }
        //     ProgramInstruction::TransferBetweenAccountsWithFee { key } => {
        //         transfer_keypair_to_account_with_fee(accounts, key)
        //     }
        //     ProgramInstruction::BurnFromAccountWithFee { key } => {
        //         burn_keypair_from_account_with_fee(accounts, key)
        //     }
    }
}
