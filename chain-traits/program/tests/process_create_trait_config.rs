// #![cfg(feature = "test-bpf")]
mod utils;

use crate::utils::{
    create_and_verify_nft, create_nft_mint, deserialize_account_info, get_trait_config,
    store_trait_config, UriMetadata,
};
use chain_traits::{
    instruction::{CreateTraitConfigArgs, TraitAction, TraitValueAction},
    state::{find_trait_config_address, TraitConfig, TraitConfigKey},
};
use solana_program::pubkey::Pubkey;
use solana_program_test::tokio;
use solana_sdk::signer::Signer;

#[tokio::test]
async fn process_create_trait_config_test_happy_path() {
    let context = &mut utils::program_test().start_with_context().await;

    let (collection_mint, _nft_token_account) = create_nft_mint(context).await;

    let collection_metadata =
        create_and_verify_nft(context, &collection_mint, None, true, None).await;

    let (nft_mint, _nft_ta) = create_nft_mint(context).await;

    let _nft_metadata =
        create_and_verify_nft(context, &nft_mint, Some(collection_mint), true, None).await;

    store_trait_config(
        context,
        &collection_mint,
        &collection_metadata.0,
        UriMetadata::get_traits(),
    )
    .await
    .unwrap();

    let (trait_config_address, _) = find_trait_config_address(&collection_mint);

    let trait_config_account = get_trait_config(context, trait_config_address).await;

    let trait_map = UriMetadata::map_traits();

    assert_eq!(
        trait_config_account
            .available_traits
            .get(&TraitConfigKey {
                id: 0,
                name: "Background".to_string()
            })
            .unwrap()
            .get(&0)
            .unwrap()
            .value,
        trait_map
            .get(&TraitConfigKey {
                id: 0,
                name: "Background".to_string()
            })
            .unwrap()
            .get(&0)
            .unwrap()
            .value
    );
}

#[tokio::test]
pub async fn process_create_config_non_collection() {
    let context = &mut utils::program_test().start_with_context().await;

    let nft_data = create_nft_mint(context).await;
    let nft_metadata = create_and_verify_nft(context, &nft_data.0, None, true, None).await;

    store_trait_config(
        context,
        &context.payer.pubkey().clone(),
        &nft_metadata.0,
        UriMetadata::get_traits(),
    )
    .await
    .unwrap();
}

#[tokio::test]
pub async fn process_create_config_non_collection_fail() {
    let context = &mut utils::program_test().start_with_context().await;

    let nft_data = create_nft_mint(context).await;
    let nft_metadata = create_and_verify_nft(context, &nft_data.0, None, true, None).await;

    store_trait_config(
        context,
        &Pubkey::new_unique(),
        &nft_metadata.0,
        UriMetadata::get_traits(),
    )
    .await
    .unwrap();
}

#[tokio::test]
pub async fn process_update_trait_config() {
    let context = &mut utils::program_test().start_with_context().await;

    let nft_data = create_nft_mint(context).await;
    let nft_metadata = create_and_verify_nft(context, &nft_data.0, None, true, None).await;

    store_trait_config(
        context,
        &nft_data.0,
        &nft_metadata.0,
        UriMetadata::get_traits(),
    )
    .await
    .unwrap();

    // let traits: Vec<CreateTraitConfigArgs> = vec![CreateTraitConfigArgs {
    //     name: "Weapon".to_string(),
    //     values: vec![
    //         TraitValueAction {
    //             name: "Sword".to_string(),
    //             action: TraitAction::Add,
    //         },
    //         TraitValueAction {
    //             name: "Gun".to_string(),
    //             action: TraitAction::Add,
    //         },
    //     ],
    // }];

    // store_trait_config(context, &nft_data.0, &nft_metadata.0, traits)
    //     .await
    //     .unwrap();

    let trait_config_address = find_trait_config_address(&nft_data.0);

    let trait_config =
        deserialize_account_info::<TraitConfig>(context, &trait_config_address.0).await;

    assert_eq!(
        trait_config
            .available_traits
            .get(&TraitConfigKey {
                name: "Weapon".to_string(),
                id: (trait_config.available_traits.len() - 1) as u8
            })
            .unwrap()
            .get(&0)
            .unwrap()
            .value,
        "Sword"
    );
}

#[tokio::test]
pub async fn process_remove_trait_from_config() {
    let context = &mut utils::program_test().start_with_context().await;

    let nft_data = create_nft_mint(context).await;
    let nft_metadata = create_and_verify_nft(context, &nft_data.0, None, true, None).await;

    store_trait_config(
        context,
        &nft_data.0,
        &nft_metadata.0,
        UriMetadata::get_traits(),
    )
    .await
    .unwrap();

    let mut traits = UriMetadata::get_traits();

    // traits.get_mut(0).unwrap().values.get_mut(0).unwrap().action = TraitAction::Remove;
    // traits.get_mut(0).unwrap().values.push(TraitValueAction {
    //     name: "Grey Goose".to_string(),
    //     action: TraitAction::Add,
    // });

    // store_trait_config(
    //     context,
    //     &nft_data.0,
    //     &nft_metadata.0,
    //     vec![
    //         traits.get(0).unwrap().clone(),
    //         CreateTraitConfigArgs {
    //             name: "Candy".to_string(),
    //             values: vec![
    //                 TraitValueAction {
    //                     name: "Gummy Bear".to_string(),
    //                     action: TraitAction::Add,
    //                 },
    //                 TraitValueAction {
    //                     name: "Sugar Candy".to_string(),
    //                     action: TraitAction::Remove,
    //                 },
    //             ],
    //         },
    //     ],
    // )
    // .await
    // .unwrap();

    let trait_config_key = find_trait_config_address(&nft_data.0);

    let trait_config_account =
        deserialize_account_info::<TraitConfig>(context, &trait_config_key.0).await;

    assert_eq!(
        trait_config_account
            .available_traits
            .get(&TraitConfigKey {
                name: traits.get(0).unwrap().name.clone(),
                id: 0
            })
            .unwrap()
            .get(&0)
            .unwrap()
            .is_active,
        false
    );
}
