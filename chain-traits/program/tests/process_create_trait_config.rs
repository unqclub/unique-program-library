// #![cfg(feature = "test-bpf")]
mod utils;
use crate::utils::{
    create_and_verify_nft, create_nft_mint, fetch_nft_json, get_trait_config, store_trait_config,
    UriMetadata,
};
use chain_traits::state::{find_trait_config_address, TraitConfig};
use solana_program_test::tokio;

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
