// #![cfg(feature = "test-bpf")]
mod utils;
use crate::utils::{
    create_and_verify_nft, create_nft_mint, get_trait_config, store_trait_config, UriMetadata,
};
use chain_traits::state::find_trait_config_address;
use solana_program::pubkey::Pubkey;
use solana_program_test::tokio;
use solana_sdk::signer::Signer;

#[tokio::test]
async fn process_create_trait_config_test_happy_path() {
    let context = &mut utils::program_test().start_with_context().await;

    let (collection_mint, _nft_token_account) = create_nft_mint(context).await;

    let collection_metadata = create_and_verify_nft(context, &collection_mint, None).await;

    let (nft_mint, _nft_ta) = create_nft_mint(context).await;

    let _nft_metadata = create_and_verify_nft(context, &nft_mint, Some(collection_mint)).await;

    store_trait_config(context, &collection_mint, &collection_metadata)
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

    store_trait_config(context, &context.payer.pubkey().clone(), &nft_metadata)
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

    store_trait_config(context, &Pubkey::new_unique(), &nft_metadata)
        .await
        .unwrap_err();
}
