//! @brief account_state manages account data

use arrayref::{array_ref, array_refs};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_error::ProgramError,
    program_memory::sol_memcpy,
    program_pack::{IsInitialized, Pack, Sealed},
};
use std::mem;

/// Currently using state. If version changes occur, this
/// should be copied to another serializable backlevel one
/// before adding new fields here
#[derive(BorshDeserialize, BorshSerialize, Debug, Default, PartialEq)]
pub struct AccountContentCurrent {
    pub somevalue: u64,
}

/// Maintains account data
#[derive(BorshDeserialize, BorshSerialize, Debug, Default, PartialEq)]
pub struct ProgramAccountState {
    is_initialized: bool,
    data_version: u8,
    account_data: AccountContentCurrent,
}

impl ProgramAccountState {
    /// Signal initialized
    pub fn set_initialized(&mut self) {
        self.is_initialized = true;
    }
    /// Get the initialized flag
    pub fn initialized(&self) -> bool {
        self.is_initialized
    }
    /// Gets the current data version
    pub fn version(&self) -> u8 {
        self.data_version
    }
    /// Get the reference to content structure
    pub fn content(&self) -> &AccountContentCurrent {
        &self.account_data
    }
    /// Get the mutable reference to content structure
    pub fn content_mut(&mut self) -> &mut AccountContentCurrent {
        &mut self.account_data
    }
}

/// Declaration of the current data version.
pub const DATA_VERSION: u8 = 0;

const IS_INITIALIZED: usize = 1;
const DATA_VERSION_ID: usize = 1;
pub const INTERMMEDIATE_SIZE: usize = IS_INITIALIZED + DATA_VERSION_ID;

pub const PREVIOUS_VERSION_DATA_SIZE: usize = mem::size_of::<AccountContentCurrent>();
pub const PREVIOUS_ACCOUNT_SPACE: usize =
    IS_INITIALIZED + DATA_VERSION_ID + PREVIOUS_VERSION_DATA_SIZE;

pub const CURRENT_VERSION_DATA_SIZE: usize = mem::size_of::<AccountContentCurrent>();
pub const ACCOUNT_STATE_SPACE: usize = IS_INITIALIZED + DATA_VERSION_ID + CURRENT_VERSION_DATA_SIZE;

pub const PROGRAM_ACCOUNT_SIZE: usize = mem::size_of::<ProgramAccountState>();

/// Future data migration logic that converts prior state of data
/// to current state of data
fn conversion_logic(src: &[u8]) -> Result<ProgramAccountState, ProgramError> {
    let past = array_ref![src, 0, PREVIOUS_ACCOUNT_SPACE];
    let (initialized, _, _account_space) = array_refs![
        past,
        IS_INITIALIZED,
        DATA_VERSION_ID,
        PREVIOUS_VERSION_DATA_SIZE
    ];
    // Logic to uplift from previous version
    // GOES HERE

    // Give back
    Ok(ProgramAccountState {
        is_initialized: initialized[0] != 0u8,
        data_version: DATA_VERSION,
        account_data: AccountContentCurrent::default(),
    })
}
impl Sealed for ProgramAccountState {}

impl IsInitialized for ProgramAccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for ProgramAccountState {
    const LEN: usize = ACCOUNT_STATE_SPACE;

    /// Store 'state' of account to its data area
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let data_out = self.try_to_vec().unwrap();
        sol_memcpy(dst, &data_out, data_out.len());
    }

    /// Retrieve 'state' of account from account data area
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let header = array_ref![src, 0, INTERMMEDIATE_SIZE];
        let initialized = header[0] != 0;
        // Check initialized
        if initialized {
            // Version check
            if header[1] == DATA_VERSION {
                // let current = array_ref![src, 0, ACCOUNT_STATE_SPACE];
                let (_, _, account_space) = array_refs![
                    array_ref![src, 0, ACCOUNT_STATE_SPACE],
                    IS_INITIALIZED,
                    DATA_VERSION_ID,
                    CURRENT_VERSION_DATA_SIZE
                ];
                Ok(ProgramAccountState {
                    is_initialized: initialized,
                    data_version: header[1],
                    account_data: AccountContentCurrent::try_from_slice(account_space).unwrap(),
                })
            } else {
                conversion_logic(src)
            }
        } else {
            Ok(ProgramAccountState {
                is_initialized: false,
                data_version: header[1],
                account_data: AccountContentCurrent::default(),
            })
        }
    }
}
