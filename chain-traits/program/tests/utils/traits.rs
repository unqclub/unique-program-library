use std::borrow::Borrow;

use chain_traits::instruction::{
    create_trait, create_trait_config, CreateTraitArgs, CreateTraitConfigArgs,
};
use chain_traits::state::TraitConfig;
use solana_program::borsh::try_from_slice_unchecked;
use solana_program::pubkey::Pubkey;
use solana_program_test::{BanksClientError, ProgramTestContext};
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

pub async fn store_trait_config(
    context: &mut ProgramTestContext,
    collection: &Pubkey,
    collection_metadata: &Pubkey,
    traits: Vec<CreateTraitConfigArgs>,
) -> Result<(), BanksClientError> {
    let instruction = create_trait_config(
        &chain_traits::id(),
        collection,
        collection_metadata,
        &context.payer.pubkey(),
        traits,
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
    payer: Option<&Keypair>,
) -> Result<(), BanksClientError> {
    let update_authority = if let Some(update_auth) = payer {
        &update_auth
    } else {
        context.payer.borrow()
    };

    let store_trait_ix = create_trait(
        &chain_traits::id(),
        nft_mint,
        nft_metadata,
        trait_config,
        &update_authority.pubkey(),
        traits,
    );

    let tx = Transaction::new_signed_with_payer(
        &[store_trait_ix],
        Some(&update_authority.pubkey()),
        &[update_authority],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await
}
