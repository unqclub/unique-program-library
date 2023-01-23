#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

pub use solana_program;
use solana_program::{
    program_memory::sol_memcmp,
    pubkey::{Pubkey, PUBKEY_BYTES},
};

solana_program::declare_id!("BVZtShyAQsxhy1uQqYFemi8NHdN8QThr8Mv5RjbW8vvQ");

pub const AUTHORIZE_SEED: &'static [u8] = b"authorize";

/// Checks two pubkeys for equality in a computationally cheap way using
/// `sol_memcmp`
pub fn cmp_pubkeys(a: &Pubkey, b: &Pubkey) -> bool {
    sol_memcmp(a.as_ref(), b.as_ref(), PUBKEY_BYTES) == 0
}

pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);

    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(&solana_program::hash::hash(preimage.as_bytes()).to_bytes()[..8]);
    sighash
}
