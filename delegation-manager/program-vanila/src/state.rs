use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

use crate::{sighash, AUTHORIZE_SEED};

#[derive(Debug, borsh::BorshDeserialize, borsh::BorshSchema, borsh::BorshSerialize)]
pub struct Delegation {
    /// Required in case anchor is used
    pub discriminator: [u8; 8],
    /// The creator of the delegation
    pub master: Pubkey,
    /// The wallet who delegates
    pub representative: Pubkey,
    /// Confirmation flag
    pub authorised: bool,
}

impl Delegation {
    pub fn new_serialized(master: Pubkey, representative: Pubkey) -> Vec<u8> {
        Self::new(master, representative)
            .try_to_vec()
            .expect("should always serialize")
    }

    pub fn new(master: Pubkey, representative: Pubkey) -> Self {
        Self {
            discriminator: sighash("account", stringify!(Delegation)),
            master,
            representative,
            authorised: false,
        }
    }
}

pub fn get_delegation_address(master: &Pubkey, representative: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &get_delegation_address_seeds(master, representative),
        &crate::ID,
    )
}

pub fn get_delegation_address_seeds<'a>(
    master: &'a Pubkey,
    representative: &'a Pubkey,
) -> [&'a [u8]; 3] {
    [AUTHORIZE_SEED, &master.as_ref(), &representative.as_ref()]
}
