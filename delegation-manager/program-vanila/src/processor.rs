use solana_program::{borsh::try_from_slice_unchecked, system_instruction::create_account};

use crate::{
    cmp_pubkeys,
    error::DelegationError,
    instruction::DelegationInstruction,
    state::{get_delegation_address, Delegation},
};

/// Program state handler.
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};
pub struct Processor {}

impl Processor {
    fn process_initialize_delegation(accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let master_info = next_account_info(account_info_iter)?;
        let representative_info = next_account_info(account_info_iter)?;
        let delegation_info = next_account_info(account_info_iter)?;

        let (delegation_address, bump) =
            get_delegation_address(master_info.key, representative_info.key);

        if !cmp_pubkeys(delegation_info.key, &delegation_address) {
            return Err(DelegationError::WrongSigner.into());
        }

        let system_program = next_account_info(account_info_iter)?;

        let delegation = Delegation::new_serialized(*master_info.key, *representative_info.key);
        let delegation_size = delegation.len();

        invoke(
            &create_account(
                master_info.key,
                delegation_info.key,
                Rent::get()?.minimum_balance(delegation_size),
                delegation_size as u64,
                &crate::ID,
            ),
            &[master_info.clone(), delegation_info.clone()],
        )?;

        delegation_info
            .try_borrow_mut_data()?
            .copy_from_slice(&delegation[..]);

        Ok(())
    }

    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction: DelegationInstruction = try_from_slice_unchecked(input)?;

        match instruction {
            DelegationInstruction::InitializeDelegation => {
                msg!("Instruction: InitializeMint");
                Self::process_initialize_delegation(accounts)
            }
            DelegationInstruction::ConfirmDelegation => todo!(),
            DelegationInstruction::CancelDelegation => todo!(),
        }
    }
}
