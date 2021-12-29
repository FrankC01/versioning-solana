//! Custom error enum

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    program_error::{PrintProgramError, ProgramError},
};
use std::fmt;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum DataVersionError {
    InvalidInstruction,
    DeserializationFailure,
    AlreadyInitializedState,
}

impl From<DataVersionError> for ProgramError {
    fn from(e: DataVersionError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for DataVersionError {
    fn type_of() -> &'static str {
        "DataVersionError"
    }
}

impl fmt::Display for DataVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataVersionError::InvalidInstruction => f.write_str("Invalid instruction"),
            DataVersionError::DeserializationFailure => {
                f.write_str("Error Deserializing input data")
            }
            DataVersionError::AlreadyInitializedState => f.write_str("Account already initialized"),
        }
    }
}

impl PrintProgramError for DataVersionError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            DataVersionError::InvalidInstruction => println!("Error: Invalid instruction"),
            DataVersionError::DeserializationFailure => println!("Error Deserializing input data"),
            DataVersionError::AlreadyInitializedState => println!("Account already initialized"),
        }
    }
}
