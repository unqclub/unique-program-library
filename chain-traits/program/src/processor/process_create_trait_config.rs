use std::collections::HashMap;
use std::ops::DerefMut;

use crate::instruction::{CreateTraitConfigArgs, TraitAction};
use crate::state::TraitConfig;
use crate::utils::{add_new_traits_bytes, create_program_account, transfer_lamports};
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
            .filter(|arg| arg.action == TraitAction::Modify)
            .collect_vec();
        if adding_traits.len() > 0 {
            let mut new_values: Vec<String> = Vec::new();
            adding_traits.iter().for_each(|add_trt| {
                add_trt
                    .values
                    .iter()
                    .for_each(|val| new_values.push(val.clone()))
            });

            let serialized_values = &new_values.try_to_vec().unwrap()[4..];

            transfer_lamports(
                update_autority,
                trait_config,
                Rent::default().minimum_balance(serialized_values.len()),
                system_program,
            )?;

            trait_config.realloc(
                trait_config.data_len() + serialized_values.len() + new_values.len(),
                false,
            )?;

            drop(serialized_values);

            let account_data = &mut trait_config.data.borrow_mut()[..];

            add_new_traits_bytes(account_data, data);
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

            if let Some(realloc_data_len) = trait_config_account
                .try_to_vec()
                .unwrap()
                .len()
                .checked_sub(data_len)
            {
                transfer_lamports(
                    update_autority,
                    trait_config,
                    Rent::default().minimum_balance(realloc_data_len),
                    system_program,
                )?;
            }

            let serialized_data = trait_config_account.try_to_vec().unwrap();

            trait_config.realloc(serialized_data.len(), false)?;
            trait_config
                .try_borrow_mut_data()?
                .copy_from_slice(&serialized_data);
        };
    }

    Ok(())
}
