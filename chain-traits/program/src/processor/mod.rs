use solana_program::{
    account_info::AccountInfo, borsh::try_from_slice_unchecked, entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::instruction::TraitInstruction;

mod process_create_trait_config;
use process_create_trait_config::*;

mod process_create_trait;
use process_create_trait::*;

pub fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    let ix = try_from_slice_unchecked::<TraitInstruction>(data).unwrap();

    match ix {
        TraitInstruction::CreateTraitConfig { data } => {
            process_create_trait_config(program_id, accounts, data)
        }
        TraitInstruction::CreateTrait { data } => process_create_trait(program_id, accounts, data),
    }
}
