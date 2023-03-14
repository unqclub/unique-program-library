use std::collections::HashMap;
use std::ops::DerefMut;

use crate::instruction::CreateTraitConfigArgs;
use crate::state::{TraitConfig, TraitConfigKey};
use crate::utils::{create_program_account, transfer_lamports};
use crate::{errors::TraitError, state::AvailableTrait};
use borsh::{BorshDeserialize, BorshSerialize};
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
#[warn(unused_assignments)]
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

    let trait_map: HashMap<TraitConfigKey, HashMap<u8, AvailableTrait>> =
        TraitConfig::traits_to_map(&data);

    if trait_config.data_is_empty() {
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
        trait_config_account.update_authority = update_autority.key.clone();
        trait_config_account.serialize(trait_config.try_borrow_mut_data()?.deref_mut())?;
    } else {
        let config_bytes = &trait_config.try_borrow_mut_data()?[76..];
        let mut new_data: Vec<u8> = Vec::new();

        new_data.extend_from_slice(&config_bytes[0..76]);

        for (key, value) in trait_map {
            let serialized_key = key.try_to_vec().unwrap();

            let serialized_value = value.try_to_vec().unwrap();

            let mut chunk_size = serialized_key.len();
        }

        // let mut trait_config_account =
        //     try_from_slice_unchecked::<TraitConfig>(&trait_config.data.borrow_mut())?;

        // msg!("DATA LEN:{:?}", trait_config.data_len());
        // // let initial_account_size = trait_config.data_len();

        // let data_iter = data.into_iter();

        // for new_trait in data_iter {
        //     let existing_trait = trait_config_account
        //         .available_traits
        //         .iter_mut()
        //         .find(|v| v.0.name == *new_trait.name);

        //     if let Some((_exiting_index, existing_trait_values)) = existing_trait {
        //         new_trait.values.iter().for_each(|(index, value)| {
        //             // existing_trait_values.insert(*index, value);
        //         });
        //     } else {
        //         trait_config_account.available_traits.insert(
        //             TraitConfigKey {
        //                 name: new_trait.name,
        //                 id: trait_config_account.available_traits.len() as u8,
        //             },
        //             new_trait.values,
        //         );
        //     };
        // }

        // let mut serialized_data: Vec<u8> = Vec::new();

        // let serialized_data = trait_config_account.try_to_vec()?;

        // trait_config_account
        //     .serialize(&mut serialized_data)
        //     .unwrap();

        // msg!("DATA SERIALIZED");
        // let mut buffer = Vec::with_capacity(6000);
        // trait_config_account.serialize(&mut buffer)?;

        // if let Some(new_len) = new_size.checked_sub(trait_config.data_len()) {
        transfer_lamports(
            update_autority,
            trait_config,
            Rent::default().minimum_balance(new_data.len()),
            system_program,
        )?;
        trait_config.realloc(new_data.len(), false)?;
        msg!("REALLOC");
        // }

        // trait_config_account.serialize(trait_config.try_borrow_mut_data()?.deref_mut())?;
        // trait_config
        //     .try_borrow_mut_data()?
        //     .copy_from_slice(&new_data);
        // config_bytes.copy_from_slice(&new_data);
        // msg!("ACC SERIALIZED");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use arrayref::{array_mut_ref, array_ref};
    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_sdk::{signature::Keypair, signer::Signer};
    use std::{
        cell::{RefCell, RefMut},
        collections::HashMap,
    };

    use crate::state::{AvailableTrait, TraitConfig, TraitConfigKey};

    #[test]
    fn moving_bytes() {
        let collection = Keypair::new();
        let update_autority = Keypair::new();
        let new_update_authority = Keypair::new();

        let mut available_traits: HashMap<TraitConfigKey, HashMap<u8, AvailableTrait>> =
            HashMap::new();
        let mut trait_values: HashMap<u8, AvailableTrait> = HashMap::new();
        trait_values.insert(
            1,
            AvailableTrait {
                value: "trait_values 1".to_string(),
                is_active: true,
            },
        );
        available_traits.insert(
            TraitConfigKey {
                name: "trait_name 1".to_string(),
                id: 1,
            },
            trait_values,
        );

        let trait_config = TraitConfig {
            collection: collection.pubkey(),
            update_authority: update_autority.pubkey(),
            last_modified: 0,
            available_traits,
        };
        println!("Old update_authority: {:#?}", update_autority.pubkey());
        println!("Original trait_config:  {:#?}", trait_config);

        let mut data = trait_config.try_to_vec().unwrap();

        // Make a RefMut from account data to simulate real solana AccountInfo.data
        let data_ref_cell: RefCell<&mut [u8]> = RefCell::new(&mut data);
        let mut account_data: RefMut<&mut [u8]> = data_ref_cell.borrow_mut();

        let old_update_authority = array_ref![account_data, 32, 32];
        assert_eq!(
            old_update_authority,
            &update_autority.pubkey().to_bytes()[..],
            "Update authority bytes are not equal"
        );

        // An example of how to find the index of a pubkey in a byte array
        // and how to swap update_authority with new update authority
        let mut index = 0;
        for i in 0..(account_data.len() - 32) {
            let temp = array_ref![account_data, i, 32];
            if temp == &update_autority.pubkey().to_bytes() {
                println!("Found at index: {}", i);
                index = i;
                break;
            }
        }

        array_mut_ref![account_data, index, 32]
            .copy_from_slice(&new_update_authority.pubkey().to_bytes());

        assert_eq!(
            new_update_authority.pubkey().to_bytes(),
            array_ref![account_data, 32, 32][..],
            "Update authority bytes are not equal"
        );

        let modified_trait_config = TraitConfig::try_from_slice(&account_data).unwrap();
        println!("New update_authority: {:#?}", new_update_authority.pubkey());
        println!("Modified trait_config: {:#?}", modified_trait_config);
    }
}
