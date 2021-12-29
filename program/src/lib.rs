//! Library declares the program-id and
//! supporting modules

pub use solana_program;

pub mod account_state;
pub mod entry_point;
mod error;
mod instruction;
pub mod processor;

solana_program::declare_id!("PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
