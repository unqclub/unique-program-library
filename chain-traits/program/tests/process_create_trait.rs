mod utils;

use std::borrow::Borrow;

use chain_traits::state::{find_trait_config_address, TraitConfig};
use solana_program::borsh::try_from_slice_unchecked;
use solana_program_test::tokio;
use utils::{
    create_and_verify_nft, create_nft_mint, fetch_nft_json, store_nft_trait, store_trait_config,
};

#[tokio::test]
pub async fn process_create_trait_happy_path() {
    let context = &mut utils::program_test().start_with_context().await;

    let (collection_mint, _) = create_nft_mint(context).await;

    let collection_metadata = create_and_verify_nft(context, &collection_mint, None).await;

    let (nft_mint, _) = create_nft_mint(context).await;

    let nft_metadata = create_and_verify_nft(context, &nft_mint, Some(collection_mint)).await;

    store_trait_config(context, &collection_mint, &collection_metadata)
        .await
        .unwrap();

    let trait_config_address = find_trait_config_address(&collection_mint).0;

    let trait_config_acc = context
        .banks_client
        .get_account(trait_config_address)
        .await
        .unwrap()
        .unwrap();
    let trait_config =
        try_from_slice_unchecked::<TraitConfig>(&trait_config_acc.data.borrow()).unwrap();

    println!("{:?}", trait_config);

    let uri_metadata = fetch_nft_json("https://metadata.y00ts.com/y/14999.json").await;

    println!("URI META:{:?}", uri_metadata);

    let create_trait_args = uri_metadata.map_to_args();

    store_nft_trait(
        context,
        &nft_mint,
        &nft_metadata,
        &trait_config_address,
        create_trait_args,
    )
    .await
    .unwrap();
}
