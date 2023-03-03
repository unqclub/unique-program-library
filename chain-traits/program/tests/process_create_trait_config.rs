// #![cfg(feature = "test-bpf")]
mod utils;
use std::borrow::Borrow;

use crate::utils::{
    create_and_verify_nft, create_nft_mint, get_trait_config, store_trait_config, UriMetadata,
};
use chain_traits::{
    instruction::{CreateTraitConfigArgs, TraitAction},
    state::{find_trait_config_address, TraitConfig},
};
use solana_program::{borsh::try_from_slice_unchecked, pubkey::Pubkey};
use solana_program_test::tokio;
use solana_sdk::signer::Signer;

#[tokio::test]
async fn process_create_trait_config_test_happy_path() {
    let context = &mut utils::program_test().start_with_context().await;

    let (collection_mint, _nft_token_account) = create_nft_mint(context).await;

    let collection_metadata = create_and_verify_nft(context, &collection_mint, None).await;

    let (nft_mint, _nft_ta) = create_nft_mint(context).await;

    let _nft_metadata = create_and_verify_nft(context, &nft_mint, Some(collection_mint)).await;

    store_trait_config(
        context,
        &collection_mint,
        &collection_metadata,
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
            .get("Background")
            .unwrap()
            .get(0)
            .unwrap()
            .value,
        trait_map.get("Background").unwrap().get(0).unwrap().value
    );
}

#[tokio::test]
pub async fn process_create_config_non_collection() {
    let context = &mut utils::program_test().start_with_context().await;

    let nft_data = create_nft_mint(context).await;
    let nft_metadata = create_and_verify_nft(context, &nft_data.0, None).await;

    store_trait_config(
        context,
        &context.payer.pubkey().clone(),
        &nft_metadata,
        UriMetadata::get_traits(),
    )
    .await
    .unwrap();

    let trait_config_address = find_trait_config_address(&context.payer.pubkey().clone());

    let trait_config_account = get_trait_config(context, trait_config_address.0).await;

    let trait_map = UriMetadata::map_traits();

    assert_eq!(
        trait_config_account
            .available_traits
            .get("Head")
            .unwrap()
            .get(0)
            .unwrap()
            .value,
        trait_map.get("Head").unwrap().get(0).unwrap().value
    );
}

#[tokio::test]
pub async fn process_create_config_non_collection_fail() {
    let context = &mut utils::program_test().start_with_context().await;

    let nft_data = create_nft_mint(context).await;
    let nft_metadata = create_and_verify_nft(context, &nft_data.0, None).await;

    store_trait_config(
        context,
        &Pubkey::new_unique(),
        &nft_metadata,
        UriMetadata::get_traits(),
    )
    .await
    .unwrap_err();
}

#[tokio::test]
pub async fn process_update_trait_config() {
    let context = &mut utils::program_test().start_with_context().await;

    let nft_data = create_nft_mint(context).await;
    let nft_metadata = create_and_verify_nft(context, &nft_data.0, None).await;

    store_trait_config(
        context,
        &nft_data.0,
        &nft_metadata,
        UriMetadata::get_traits(),
    )
    .await
    .unwrap();

    let traits: Vec<CreateTraitConfigArgs> = vec![CreateTraitConfigArgs {
        action: TraitAction::Add,
        name: "Weapon".to_string(),
        values: vec!["Sword".to_string(), "Gun".to_string()],
    }];

    store_trait_config(context, &nft_data.0, &nft_metadata, traits)
        .await
        .unwrap();

    let trait_config_address = find_trait_config_address(&nft_data.0);

    let trait_config_account = context
        .banks_client
        .get_account(trait_config_address.0)
        .await
        .unwrap()
        .unwrap();

    let trait_config: TraitConfig =
        try_from_slice_unchecked(&trait_config_account.data.borrow()).unwrap();

    println!("{:?}", trait_config);
}
