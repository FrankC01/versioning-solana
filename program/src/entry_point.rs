//! entry point for instruction execution

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    program_error::PrintProgramError, pubkey::Pubkey,
};

use crate::{error::DataVersionError, processor::process};

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = process(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<DataVersionError>();
        return Err(error);
    }
    Ok(())
}
