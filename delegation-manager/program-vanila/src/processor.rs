use solana_program::{borsh::try_from_slice_unchecked, system_instruction::create_account};

use crate::{
    cmp_pubkeys,
    error::DelegationError,
    instruction::DelegationInstruction,
    state::{get_delegation_address, get_delegation_address_seeds, Delegation},
};

/// Program state handler.
use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::{invoke, invoke_signed, set_return_data},
        pubkey::Pubkey,
        system_instruction, system_program,
        sysvar::{rent::Rent, Sysvar},
    },
    std::convert::{TryFrom, TryInto},
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
        //TODO bump

        // let new_account_info = next_account_info(account_info_iter)?;
        // let mint_info = next_account_info(account_info_iter)?;
        // let owner = if let Some(owner) = owner {
        //     owner
        // } else {
        //     next_account_info(account_info_iter)?.key
        // };
        // let new_account_info_data_len = new_account_info.data_len();
        // let rent = if rent_sysvar_account {
        //     Rent::from_account_info(next_account_info(account_info_iter)?)?
        // } else {
        //     Rent::get()?
        // };

        // let mut account_data = new_account_info.data.borrow_mut();
        // // unpack_uninitialized checks account.base.is_initialized() under the hood
        // let mut account =
        //     StateWithExtensionsMut::<Account>::unpack_uninitialized(&mut account_data)?;

        // if !rent.is_exempt(new_account_info.lamports(), new_account_info_data_len) {
        //     return Err(TokenError::NotRentExempt.into());
        // }

        // // get_required_account_extensions checks mint validity
        // let mint_data = mint_info.data.borrow();
        // let mint = StateWithExtensions::<Mint>::unpack(&mint_data)
        //     .map_err(|_| Into::<ProgramError>::into(TokenError::InvalidMint))?;
        // if mint
        //     .get_extension::<PermanentDelegate>()
        //     .map(|e| Option::<Pubkey>::from(e.delegate).is_some())
        //     .unwrap_or(false)
        // {
        //     msg!("Warning: Mint has a permanent delegate, so tokens in this account may be seized at any time");
        // }
        // let required_extensions =
        //     Self::get_required_account_extensions_from_unpacked_mint(mint_info.owner, &mint)?;
        // if ExtensionType::get_account_len::<Account>(&required_extensions)
        //     > new_account_info_data_len
        // {
        //     return Err(ProgramError::InvalidAccountData);
        // }
        // for extension in required_extensions {
        //     account.init_account_extension_from_type(extension)?;
        // }

        // let starting_state =
        //     if let Ok(default_account_state) = mint.get_extension::<DefaultAccountState>() {
        //         AccountState::try_from(default_account_state.state)
        //             .or(Err(ProgramError::InvalidAccountData))?
        //     } else {
        //         AccountState::Initialized
        //     };

        // account.base.mint = *mint_info.key;
        // account.base.owner = *owner;
        // account.base.close_authority = COption::None;
        // account.base.delegate = COption::None;
        // account.base.delegated_amount = 0;
        // account.base.state = starting_state;
        // if cmp_pubkeys(mint_info.key, &native_mint::id()) {
        //     let rent_exempt_reserve = rent.minimum_balance(new_account_info_data_len);
        //     account.base.is_native = COption::Some(rent_exempt_reserve);
        //     account.base.amount = new_account_info
        //         .lamports()
        //         .checked_sub(rent_exempt_reserve)
        //         .ok_or(TokenError::Overflow)?;
        // } else {
        //     account.base.is_native = COption::None;
        //     account.base.amount = 0;
        // };

        // account.pack_base();
        // account.init_account_type()?;

        //Err(ProgramError::InvalidAccountData)
        // TokenError::Overflow
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
