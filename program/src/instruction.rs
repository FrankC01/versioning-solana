//! instruction Contains the main ProgramInstruction enum

use {
    crate::error::DataVersionError, borsh::BorshDeserialize,
    solana_program::program_error::ProgramError,
};

/// Generic Payload Deserialization
#[derive(BorshDeserialize, Debug)]
struct Payload {
    variant: u8,
}

#[derive(Debug, PartialEq)]
/// All custom program instructions
pub enum ProgramInstruction {
    InitializeAccount,
}

impl ProgramInstruction {
    /// Unpack inbound buffer to associated Instruction
    /// The expected format for input is a Borsh serialized vector
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let payload = Payload::try_from_slice(input).unwrap();
        match payload.variant {
            0 => Ok(ProgramInstruction::InitializeAccount),
            _ => Err(DataVersionError::InvalidInstruction.into()),
        }
    }
}
