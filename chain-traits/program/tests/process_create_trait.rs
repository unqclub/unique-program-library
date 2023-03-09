mod utils;

use crate::utils::UriMetadata;
use chain_traits::state::{
    find_trait_config_address, find_trait_data_address, TraitConfig, TraitData,
};
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_sdk::{signature::Keypair, signer::Signer};

use solana_program_test::tokio;
use utils::{
    airdrop_funds, create_account, create_and_verify_nft, create_nft_mint,
    deserialize_account_info, fetch_nft_json, mint_and_store_trait, store_nft_trait,
    store_trait_config,
};

#[tokio::test]
pub async fn process_create_trait_happy_path() {
    let context = &mut utils::program_test().start_with_context().await;

    let (collection_mint, _) = create_nft_mint(context).await;

    let collection_metadata =
        create_and_verify_nft(context, &collection_mint, None, true, None).await;

    let (nft_mint, _) = create_nft_mint(context).await;

    let nft_metadata =
        create_and_verify_nft(context, &nft_mint, Some(collection_mint), true, None).await;

    store_trait_config(
        context,
        &collection_mint,
        &collection_metadata.0,
        UriMetadata::get_traits(),
    )
    .await
    .unwrap();

    let trait_config_address = find_trait_config_address(&collection_mint).0;
    let trait_config_account =
        deserialize_account_info::<TraitConfig>(context, &trait_config_address).await;

    let uri_metadata = fetch_nft_json("https://metadata.y00ts.com/y/14999.json").await;

    let create_trait_args = uri_metadata.map_to_args(trait_config_account);

    store_nft_trait(
        context,
        &nft_mint,
        &nft_metadata.0,
        &trait_config_address,
        vec![create_trait_args],
        None,
    )
    .await
    .unwrap();

    let trait_data_account = deserialize_account_info::<TraitData>(
        context,
        &find_trait_data_address(&trait_config_address, &nft_mint).0,
    )
    .await;

    assert_eq!(trait_data_account.nft_mint, nft_mint);

    assert_eq!(*trait_data_account.traits.get(&0).unwrap(), 2 as u8);
}

#[tokio::test]
pub async fn process_save_traits_non_update_authority() {
    let minter = Keypair::new();
    let transfer_amount = 10_u64.checked_mul(LAMPORTS_PER_SOL).unwrap();
    let context = &mut utils::program_test().start_with_context().await;

    let (collection_mint, _) = create_nft_mint(context).await;
    let collection_metadata =
        create_and_verify_nft(context, &collection_mint, None, true, None).await;

    store_trait_config(
        context,
        &collection_mint,
        &collection_metadata.0,
        UriMetadata::get_traits(),
    )
    .await
    .unwrap();
    let trait_config_address = find_trait_config_address(&collection_mint);

    create_account(context, &minter).await;
    airdrop_funds(context, &minter.pubkey(), transfer_amount).await;

    assert_eq!(
        context
            .banks_client
            .get_balance(minter.pubkey())
            .await
            .unwrap(),
        transfer_amount
    );

    let (nft_mint, _) = create_nft_mint(context).await;
    let nft_metadata =
        create_and_verify_nft(context, &nft_mint, Some(collection_mint), true, None).await;

    let y00t_traits = fetch_nft_json("https://metadata.y00ts.com/y/14999.json").await;

    let trait_config_account =
        deserialize_account_info::<TraitConfig>(context, &trait_config_address.0).await;

    store_nft_trait(
        context,
        &nft_mint,
        &nft_metadata.0,
        &trait_config_address.0,
        vec![y00t_traits.map_to_args(trait_config_account)],
        Some(&minter),
    )
    .await
    .unwrap_err();
}

#[tokio::test]
pub async fn process_store_trait_when_minting() {
    let context = &mut utils::program_test().start_with_context().await;
    let (collection_mint, _) = create_nft_mint(context).await;
    let (collection_metadata, _) =
        create_and_verify_nft(context, &collection_mint, None, true, None).await;

    let minter = Keypair::new();

    create_account(context, &minter).await;

    airdrop_funds(
        context,
        &minter.pubkey(),
        5_u64.checked_mul(LAMPORTS_PER_SOL).unwrap(),
    )
    .await;

    store_trait_config(
        context,
        &collection_mint,
        &collection_metadata,
        UriMetadata::get_traits(),
    )
    .await
    .unwrap();

    let (trait_config_address, _bump) = find_trait_config_address(&collection_mint);

    let traits = fetch_nft_json("https://metadata.y00ts.com/y/14999.json").await;

    let trait_config_account =
        deserialize_account_info::<TraitConfig>(context, &trait_config_address).await;

    mint_and_store_trait(
        context,
        &collection_mint,
        &trait_config_address,
        vec![traits.map_to_args(trait_config_account)],
        &minter,
    )
    .await;
}
