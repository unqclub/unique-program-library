use std::collections::HashMap;
use std::ops::DerefMut;

use crate::instruction::{CreateTraitConfigArgs, TraitAction};
use crate::state::TraitConfig;
use crate::utils::{create_program_account, transfer_lamports};
use crate::{errors::TraitError, state::AvailableTrait};
use borsh::BorshSerialize;
use mpl_token_metadata::state::{Metadata, TokenMetadataAccount};
use solana_program::borsh::try_from_slice_unchecked;
use solana_program::clock::Clock;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};
use spl_token::ID as TOKEN_PROGRAM_ID;
pub fn create_trait_config<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: Vec<CreateTraitConfigArgs>,
) -> ProgramResult {
    let account_infos = &mut accounts.iter();

    let collection = next_account_info(account_infos)?;
    let trait_config = next_account_info(account_infos)?;
    let update_autority = next_account_info(account_infos)?;
    let collection_metadata = next_account_info(account_infos)?;
    let system_program = next_account_info(account_infos)?;

    let collection_metadata_account = Metadata::from_account_info(collection_metadata)?;

    if collection.owner.clone() == TOKEN_PROGRAM_ID {
        assert!(
            collection_metadata_account.collection.unwrap().key == collection.key.clone(),
            "{}",
            TraitError::InvalidCollection
        );
    } else {
        assert!(
            collection_metadata_account
                .data
                .creators
                .unwrap()
                .get(0)
                .unwrap()
                .address
                == *collection.key,
            "{}",
            TraitError::InvalidCollection
        );
    }

    assert!(
        collection_metadata_account.update_authority == *update_autority.key,
        "{}",
        TraitError::NotUpdateAuthority
    );

    let (trait_config_account_address, trait_config_account_bump) = Pubkey::find_program_address(
        &TraitConfig::get_trait_config_seeds(collection.key),
        program_id,
    );

    assert!(
        trait_config_account_address == *trait_config.key,
        "{}",
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
        let mut trait_config_account =
            try_from_slice_unchecked::<TraitConfig>(&trait_config.data.borrow_mut())?;

        let data_len = trait_config.data_len();

        for new_trait in data.iter() {
            trait_config_account.available_traits.insert(
                new_trait.name.clone(),
                TraitConfig::map_available_traits(
                    new_trait.values.clone(),
                    new_trait.action == TraitAction::Add,
                ),
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

    Ok(())
}
