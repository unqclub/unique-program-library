use std::{collections::HashMap, ops::DerefMut};

use borsh::BorshSerialize;
use itertools::Itertools;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    clock::Clock,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::{
        instructions::{load_current_index_checked, load_instruction_at_checked},
        Sysvar,
    },
};

use crate::{
    errors::TraitError,
    instruction::CreateTraitArgs,
    state::{TraitConfig, TraitData},
    utils::{create_program_account, transfer_lamports},
};

pub fn process_create_trait<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: Vec<Vec<CreateTraitArgs>>,
) -> ProgramResult {
    let account_infos = &mut accounts.iter();
    let trait_config_account_info = next_account_info(account_infos)?;
    let payer = next_account_info(account_infos)?;
    let system_program = next_account_info(account_infos)?;
    let instructon_sysvar = next_account_info(account_infos)?;

    assert!(
        !trait_config_account_info.data_is_empty(),
        "{:?}",
        TraitError::TraitConfigNotInitialized
    );

    // let metadata_acc = Metadata::from_account_info(nft_metadata_info)?;

    // let metadata_address = find_metadata_account(nft_mint_info.key).0;

    // assert!(
    //     metadata_address == *nft_metadata_info.key,
    //     "{:?}",
    //     TraitError::InvalidCollection
    // );

    let trait_config =
        try_from_slice_unchecked::<TraitConfig>(&trait_config_account_info.data.borrow())?;

    // if let Some(collection) = metadata_acc.collection {
    //     assert!(
    //         collection.key == trait_config.collection,
    //         "{:?}",
    //         TraitError::InvalidCollection
    //     );
    // } else {
    //     assert!(
    //         metadata_acc.data.creators.unwrap().get(0).unwrap().address == trait_config.collection,
    //         "{:?}",
    //         TraitError::InvalidCollection
    //     );
    // }

    for (index, (_nft_metadata, trait_account_info, nft_mint_info)) in
        account_infos.tuples().enumerate()
    {
        if *payer.key != trait_config.update_authority {
            let instruction_index = load_current_index_checked(instructon_sysvar)?;

            let mut mint_to_executed = false;
            for i in 0..instruction_index {
                let instruction = load_instruction_at_checked(i.into(), instructon_sysvar)?;
                if instruction.program_id == spl_token::id()
                    && instruction.accounts.get(0).unwrap().pubkey == *nft_mint_info.key
                {
                    mint_to_executed = true;
                    break;
                }
            }

            assert!(
                mint_to_executed,
                "{:?}",
                TraitError::WrongAuthorityToCreateTrait
            );
        }

        let mut trait_map: HashMap<u8, u8> = HashMap::new();
        for trait_data in data.get(index).unwrap() {
            if let Some(available_trait) = trait_config
                .available_traits
                .iter()
                .find(|trait_info| trait_info.0.id == trait_data.name)
            {
                assert!(
                    available_trait.1.get(&trait_data.value).is_some(),
                    "{:?}",
                    TraitError::TraitDoesNotExist
                );
            } else {
                return Err(TraitError::TraitDoesNotExist.into());
            }
            trait_map.insert(trait_data.name.clone(), trait_data.value.clone());
        }
        let (trait_account_address, trait_account_bump) = Pubkey::find_program_address(
            &TraitData::get_trait_data_seeds(nft_mint_info.key, trait_config_account_info.key),
            program_id,
        );

        assert!(
            trait_account_address == *trait_account_info.key,
            "{:?}",
            TraitError::InvalidAccountSeeds
        );

        if trait_account_info.data_is_empty() {
            create_program_account(
                payer,
                trait_account_info,
                Some(&[
                    b"trait-data",
                    nft_mint_info.key.as_ref(),
                    trait_config_account_info.key.as_ref(),
                    &[trait_account_bump],
                ]),
                program_id,
                (TraitData::LEN + trait_map.try_to_vec().unwrap().len()) as u64,
                system_program,
            )?;
            let mut trait_account =
                try_from_slice_unchecked::<TraitData>(&trait_account_info.data.borrow())?;
            trait_account.nft_mint = *nft_mint_info.key;
            trait_account.last_modified = Clock::get().unwrap().unix_timestamp;
            trait_account.traits = trait_map;
            trait_account.trait_config = *trait_config_account_info.key;
            trait_account.serialize(trait_account_info.try_borrow_mut_data()?.deref_mut())?;
        } else {
            let mut trait_account =
                try_from_slice_unchecked::<TraitData>(&trait_account_info.data.borrow())?;
            trait_map.iter_mut().for_each(|(key, value)| {
                trait_account.traits.insert(key.clone(), value.clone());
            });
            let realloc_data_len = trait_account
                .try_to_vec()
                .unwrap()
                .len()
                .checked_sub(trait_account_info.data_len())
                .unwrap();
            if realloc_data_len > 0 {
                transfer_lamports(
                    payer,
                    trait_account_info,
                    Rent::default().minimum_balance(realloc_data_len),
                    system_program,
                )?;
                trait_account_info.realloc(trait_account.try_to_vec().unwrap().len(), false)?;
                trait_account.serialize(trait_account_info.try_borrow_mut_data()?.deref_mut())?;
            }
        }
    }

    Ok(())
}
