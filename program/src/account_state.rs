//! @brief account_state manages account data

use arrayref::{array_ref, array_refs};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_error::ProgramError,
    program_memory::sol_memcpy,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};
use std::mem;

/// Declaration of the data version. This
/// constant changes
pub const DATA_VERSION: u8 = 0;

/// Currently using state. If version changes occur, this
/// should be copied to another serializable backlevel one
/// before adding new fields here
#[derive(BorshDeserialize, BorshSerialize, Debug, Default, PartialEq)]
pub struct AccountContentCurrent {
    pub somekey: Pubkey,
}

// impl AccountContentCurrent {
//     pub fn set_key(&mut self, other_key: Pubkey) {
//         self.somekey = other_key
//     }
// }
/// Maintains global accumulator
#[derive(BorshDeserialize, BorshSerialize, Debug, Default, PartialEq)]
pub struct ProgramAccountState {
    pub is_initialized: bool,
    pub data_version: u8,
    pub account_data: AccountContentCurrent,
}

impl ProgramAccountState {
    ///
    pub fn set_initialized(&mut self) {
        self.is_initialized = true;
    }
    pub fn get_content(&self) -> &AccountContentCurrent {
        &self.account_data
    }
    pub fn get_content_mut(&mut self) -> &mut AccountContentCurrent {
        &mut self.account_data
    }
}

const IS_INITIALIZED: usize = 1;
const DATA_VERSION_ID: usize = 1;
pub const INTERMMEDIATE_SIZE: usize = IS_INITIALIZED + DATA_VERSION_ID;

pub const PREVIOUS_VERSION_DATA_SIZE: usize = mem::size_of::<AccountContentCurrent>();
pub const PREVIOUS_ACCOUNT_SPACE: usize =
    IS_INITIALIZED + DATA_VERSION_ID + PREVIOUS_VERSION_DATA_SIZE;
pub const CURRENT_VERSION_DATA_SIZE: usize = mem::size_of::<AccountContentCurrent>();
pub const ACCOUNT_STATE_SPACE: usize = IS_INITIALIZED + DATA_VERSION_ID + CURRENT_VERSION_DATA_SIZE;

/// Future data migration logic that converts prior state of data
/// to current state of data
fn conversion_logic(src: &[u8]) -> Result<ProgramAccountState, ProgramError> {
    let past = array_ref![src, 0, PREVIOUS_ACCOUNT_SPACE];
    let (initialized, dversion, _account_space) = array_refs![
        past,
        IS_INITIALIZED,
        DATA_VERSION_ID,
        PREVIOUS_VERSION_DATA_SIZE
    ];
    Ok(ProgramAccountState {
        is_initialized: initialized[0] != 0u8,
        data_version: dversion[0],
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
                let current = array_ref![src, 0, ACCOUNT_STATE_SPACE];
                let (_, _, account_space) = array_refs![
                    current,
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
                is_initialized: initialized,
                data_version: header[1],
                account_data: AccountContentCurrent::default(),
            })
        }
    }
}
