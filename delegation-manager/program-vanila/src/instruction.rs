use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::system_program::ID as SYSTEM_PROGRAM_ID;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::state::get_delegation_address;

#[derive(Clone, Debug, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum DelegationInstruction {
    InitializeDelegation,
    ConfirmDelegation,
    CancelDelegation,
}

pub fn initialize_delegation(
    program_id: &Pubkey,
    // Accounts
    master: &Pubkey,
    representative: &Pubkey,
) -> Instruction {
    let delegation = get_delegation_address(master, representative);

    let accounts = vec![
        AccountMeta::new(*master, true),
        AccountMeta::new_readonly(*representative, false),
        AccountMeta::new(delegation, false),
        AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
    ];

    let instruction = DelegationInstruction::InitializeDelegation;

    Instruction {
        program_id: *program_id,
        accounts,
        data: instruction.try_to_vec().unwrap(),
    }
}
