// #![cfg(feature = "test-bpf")]
mod utils;
use std::borrow::Borrow;

use mpl_token_metadata::state::Metadata;
use solana_program::borsh::try_from_slice_unchecked;
use solana_program_test::tokio;

use crate::utils::{create_and_verify_collection, create_nft_mint};
#[tokio::test]
async fn process_create_trait_config_test() {
    let context = &mut utils::program_test().start_with_context().await;

    let (nft_mint, _nft_token_account) = create_nft_mint(context).await;

    let metadata_acc = create_and_verify_collection(context, &nft_mint).await;

    let meta_acc_info = context
        .banks_client
        .get_account(metadata_acc)
        .await
        .unwrap()
        .unwrap();

    let metadata = try_from_slice_unchecked::<Metadata>(&meta_acc_info.data.borrow()).unwrap();

    println!("{:?}", metadata);
}
