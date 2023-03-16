use std::collections::HashMap;
use std::ops::DerefMut;

use crate::instruction::{CreateTraitConfigArgs, TraitAction};
use crate::state::TraitConfig;
use crate::utils::{
    calculate_array_length, create_program_account, get_u32_from_slice, transfer_lamports,
};
use crate::{errors::TraitError, state::AvailableTrait};
use borsh::BorshSerialize;
use itertools::Itertools;
use solana_program::borsh::try_from_slice_unchecked;
use solana_program::clock::Clock;

use solana_program::msg;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};
pub fn process_create_trait_config<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: Vec<CreateTraitConfigArgs>,
) -> ProgramResult {
    let account_infos = &mut accounts.iter();

    let collection = next_account_info(account_infos)?;
    let trait_config = next_account_info(account_infos)?;
    let update_autority = next_account_info(account_infos)?;
    let _collection_metadata = next_account_info(account_infos)?;
    let system_program = next_account_info(account_infos)?;

    //TODO:comment in before mainnet

    // let collection_metadata_account = Metadata::from_account_info(collection_metadata)?;

    // if collection.owner.clone() != TOKEN_PROGRAM_ID {
    //     assert!(
    //         collection_metadata_account
    //             .data
    //             .creators
    //             .unwrap()
    //             .get(0)
    //             .unwrap()
    //             .address
    //             == *collection.key,
    //         "{}",
    //         TraitError::InvalidCollection
    //     );
    // }

    // assert!(
    //     collection_metadata_account.update_authority == *update_autority.key,
    //     "{}",
    //     TraitError::NotUpdateAuthority
    // );

    let (trait_config_account_address, trait_config_account_bump) = Pubkey::find_program_address(
        &TraitConfig::get_trait_config_seeds(collection.key),
        program_id,
    );

    assert!(
        trait_config_account_address == *trait_config.key,
        "{:?}",
        TraitError::InvalidAccountSeeds
    );
    if trait_config.data_is_empty() {
        let trait_map: HashMap<String, Vec<AvailableTrait>> = TraitConfig::traits_to_map(data);

        let trait_map_len = trait_map.try_to_vec().unwrap().len();
        create_program_account(
            update_autority,
            trait_config,
            Some(&[
                b"trait-config",
                collection.key.as_ref(),
                &[trait_config_account_bump],
            ]),
            program_id,
            (TraitConfig::LEN + trait_map_len) as u64,
            system_program,
        )?;

        let mut trait_config_account =
            try_from_slice_unchecked::<TraitConfig>(&trait_config.data.borrow_mut())?;

        trait_config_account.available_traits = trait_map;
        trait_config_account.collection = collection.key.clone();
        trait_config_account.last_modified = Clock::get().unwrap().unix_timestamp;
        trait_config_account.update_authoirty = update_autority.key.clone();
        trait_config_account.serialize(trait_config.try_borrow_mut_data()?.deref_mut())?;
    } else {
        let adding_traits = data
            .iter()
            .filter(|arg| arg.action == TraitAction::Modify)
            .collect_vec();
        let (new_len, new_data) = if adding_traits.len() > 0 {
            let account_data = &*trait_config.data.borrow();
            let existing_data = &account_data[76..];

            let mut new_account_len: usize = 0;
            let mut new_data: Vec<u8> = Vec::new();
            for arg in data.iter() {
                let serialized_arg_name = arg.name.try_to_vec().unwrap();
                let mut index = 0;

                loop {
                    if index >= existing_data.len() {
                        break;
                    }
                    let key = &existing_data[index..index + serialized_arg_name.len()];
                    let key_length = get_u32_from_slice(&key[0..4]) as usize;
                    let array_length = get_u32_from_slice(
                        &existing_data[index + key_length + 4..index + key_length + 8],
                    ) as usize;

                    let array_bytes = calculate_array_length(
                        &existing_data[index + key_length + 8..],
                        array_length,
                    );

                    let existing_array = &existing_data
                        [index + key_length + 8..index + key_length + 8 + array_bytes];

                    if key == serialized_arg_name {
                        new_data.extend_from_slice(&key);
                        let new_len = (array_length + arg.values.len()).to_le_bytes();
                        new_data.extend_from_slice(&new_len);
                        new_data.extend_from_slice(&existing_array);
                        let mapped_values: Vec<AvailableTrait> = arg
                            .values
                            .iter()
                            .map(|val| AvailableTrait {
                                is_active: true,
                                value: val.clone(),
                            })
                            .collect();
                        let serialized_values = &mapped_values.try_to_vec().unwrap()[4..];

                        msg!("SERIALIZED LEN:{:?}", serialized_values.len());

                        new_account_len += serialized_values.len();

                        new_data.extend_from_slice(&serialized_values);
                    } else {
                        new_data.extend_from_slice(&key_length.to_le_bytes());
                        new_data.extend_from_slice(&array_length.to_le_bytes());
                        new_data.extend_from_slice(&existing_array);
                    }
                    index += 4 + key_length + 4 + array_bytes;
                }
            }

            let mut new_account_data: Vec<u8> = Vec::new();
            new_account_data.extend_from_slice(&account_data[0..76]);
            new_account_data.extend_from_slice(&new_data);

            (new_account_len, new_account_data)
        } else {
            let mut trait_config_account =
                try_from_slice_unchecked::<TraitConfig>(&trait_config.data.borrow_mut())?;

            let data_len = trait_config.data_len();

            for new_trait in data.iter() {
                trait_config_account.update_traits(
                    &new_trait.values,
                    &new_trait.action,
                    &new_trait.name,
                );
            }

            let realloc_data_len = trait_config_account
                .try_to_vec()
                .unwrap()
                .len()
                .checked_sub(data_len)
                .unwrap();
            (realloc_data_len, trait_config_account.try_to_vec().unwrap())
        };

        transfer_lamports(
            update_autority,
            trait_config,
            Rent::default().minimum_balance(new_len),
            system_program,
        )?;

        trait_config.realloc(new_data.len(), false)?;

        trait_config
            .try_borrow_mut_data()?
            .copy_from_slice(&new_data);
    }

    Ok(())
}
