// #![cfg(feature = "test-bpf")]
mod utils;
use crate::utils::{create_and_verify_nft, create_nft_mint, fetch_nft_json};
use solana_program::sysvar::{instructions::Instructions, Sysvar};
use solana_program_test::tokio;

#[tokio::test]
async fn process_create_trait_config_test() {
    let context = &mut utils::program_test().start_with_context().await;

    let (collection_mint, _nft_token_account) = create_nft_mint(context).await;

    let _metadata_acc = create_and_verify_nft(context, &collection_mint, None).await;

    let (nft_mint, _nft_ta) = create_nft_mint(context).await;

    let _nft_metadata = create_and_verify_nft(context, &nft_mint, Some(collection_mint)).await;

    let traits = fetch_nft_json("https://metadata.y00ts.com/y/14985.json").await;

    assert_eq!(traits.attributes.get(0).unwrap().trait_type, "Background");
}
