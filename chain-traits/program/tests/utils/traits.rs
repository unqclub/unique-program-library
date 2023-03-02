use std::borrow::Borrow;

use chain_traits::instruction::{create_trait, create_trait_config, CreateTraitArgs};
use chain_traits::state::TraitConfig;
use solana_program::borsh::try_from_slice_unchecked;
use solana_program::pubkey::Pubkey;
use solana_program_test::{BanksClientError, ProgramTestContext};
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

use super::nft_state::UriMetadata;

pub async fn store_trait_config(
    context: &mut ProgramTestContext,
    collection: &Pubkey,
    collection_metadata: &Pubkey,
) -> Result<(), BanksClientError> {
    let instruction = create_trait_config(
        &chain_traits::id(),
        collection,
        collection_metadata,
        &context.payer.pubkey(),
        UriMetadata::get_traits(),
    );
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await
}

pub async fn get_trait_config(
    context: &mut ProgramTestContext,
    trait_config_address: Pubkey,
) -> TraitConfig {
    let account = context
        .banks_client
        .get_account(trait_config_address)
        .await
        .unwrap()
        .unwrap();

    try_from_slice_unchecked::<TraitConfig>(&account.data.borrow()).unwrap()
}

pub async fn store_nft_trait(
    context: &mut ProgramTestContext,
    nft_mint: &Pubkey,
    nft_metadata: &Pubkey,
    trait_config: &Pubkey,
    traits: Vec<CreateTraitArgs>,
) -> Result<(), BanksClientError> {
    let store_trait_ix = create_trait(
        &chain_traits::id(),
        nft_mint,
        nft_metadata,
        trait_config,
        &context.payer.pubkey(),
        traits,
    );

    let tx = Transaction::new_signed_with_payer(
        &[store_trait_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await
}
