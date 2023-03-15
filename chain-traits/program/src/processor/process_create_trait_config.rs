use std::collections::HashMap;
use std::ops::DerefMut;

use crate::instruction::{CreateTraitConfigArgs, TraitAction};
use crate::state::TraitConfig;
use crate::utils::{create_program_account, transfer_lamports};
use crate::{errors::TraitError, state::AvailableTrait};
use borsh::BorshSerialize;
use itertools::Itertools;
use solana_program::borsh::try_from_slice_unchecked;
use solana_program::clock::Clock;
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
            .filter(|arg| arg.action == TraitAction::Add)
            .collect_vec();
        if adding_traits.len() > 0 {
            let account_data = &trait_config.data.borrow();
            let existing_data = &account_data[76..];
            let fixed_bytes = &account_data[0..76];

            let mut new_len: usize = 0;
            let mut new_data: Vec<u8> = Vec::new();
            for arg in data.iter() {
                let serialized_arg_name = arg.name.try_to_vec().unwrap();

                for mut index in 0..existing_data.len() {
                    let key = &existing_data[index..serialized_arg_name.len()];
                    if key == serialized_arg_name {
                        new_data.extend_from_slice(key);
                        let current_vec_len = existing_data[key.len() + index + 1];
                        new_data.extend_from_slice(&[current_vec_len + arg.values.len() as u8]);
                        let old_vec_start = key.len() + index + 2 as usize;
                        let old_vec_end = existing_data[key.len() + index + 1] as usize;
                        let curr_vec = &existing_data[old_vec_start..old_vec_end];
                        new_data.extend_from_slice(curr_vec);
                        let new_traits: Vec<AvailableTrait> = arg
                            .values
                            .iter()
                            .map(|arg| AvailableTrait {
                                is_active: true,
                                value: arg.clone(),
                            })
                            .collect();
                        let serialized_new_traits = new_traits.try_to_vec().unwrap();
                        new_len += serialized_new_traits.len();
                        new_data.extend_from_slice(&serialized_new_traits);
                    } else {
                        new_data.extend_from_slice(key);
                        let vec_length = usize::from(existing_data[index + key.len() + 1]);
                        new_data
                            .extend_from_slice(&existing_data[index + key.len() + 1..vec_length]);
                    }

                    index += key.len() + usize::from(existing_data[key.len() + index + 1]);
                }
            }
            transfer_lamports(
                update_autority,
                trait_config,
                Rent::default().minimum_balance(new_len),
                system_program,
            )?;
            let mut new_account_data: Vec<u8> = Vec::new();
            new_account_data.extend_from_slice(&fixed_bytes);
            new_account_data.extend_from_slice(&new_data);

            trait_config.realloc(new_data.len(), false)?;

            trait_config
                .try_borrow_mut_data()?
                .copy_from_slice(&new_data);
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

            transfer_lamports(
                update_autority,
                trait_config,
                Rent::default().minimum_balance(realloc_data_len),
                system_program,
            )?;

            trait_config.realloc(trait_config_account.try_to_vec().unwrap().len(), false)?;

            trait_config_account.serialize(trait_config.try_borrow_mut_data()?.deref_mut())?;
        }
    }

    Ok(())
}
