//! Library declares the program-id and
//! supporting modules

pub use solana_program;

pub mod account_state;
pub mod entry_point;
pub mod error;
pub mod instruction;
pub mod processor;

solana_program::declare_id!("PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc");
