// #![cfg(feature = "test-bpf")]

use std::borrow::Borrow;

use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::pda::find_master_edition_account;
use mpl_token_metadata::state::{Collection, CollectionDetails, Creator};
use mpl_token_metadata::ID as METADATA_PROGRAM;
use mpl_token_metadata::{instruction::create_master_edition_v3, pda::find_metadata_account};
use solana_program::borsh::try_from_slice_unchecked;
use solana_program::instruction::Instruction;
use solana_program::{
    program_pack::Pack, pubkey::Pubkey, rent::Rent, system_instruction::create_account,
};
use solana_program_test::*;
use solana_sdk::{
    signer::{keypair::Keypair, Signer},
    transaction::Transaction,
};
use spl_token::instruction::initialize_account;
use spl_token::state::Account;
use spl_token::{state::Mint, ID};

use super::nft_state::UriMetadata;

pub async fn create_nft_mint(context: &mut ProgramTestContext) -> (Pubkey, Pubkey) {
    let mint = Keypair::new();

    let create_account_ix = create_account(
        &context.payer.pubkey(),
        &mint.pubkey(),
        Rent::default().minimum_balance(Mint::LEN),
        Mint::LEN as u64,
        &ID,
    );

    let ix = spl_token::instruction::initialize_mint(
        &ID,
        &mint.pubkey(),
        &context.payer.pubkey(),
        Some(&context.payer.pubkey()),
        0,
    )
    .unwrap();

    let token_account_key = Keypair::new();

    let create_token_acc_ix = create_account(
        &context.payer.pubkey(),
        &token_account_key.pubkey(),
        Rent::default().minimum_balance(Account::LEN),
        Account::LEN as u64,
        &ID,
    );

    let initialize_token_account = initialize_account(
        &spl_token::ID,
        &token_account_key.pubkey(),
        &mint.pubkey(),
        &context.payer.pubkey(),
    )
    .unwrap();

    let create_ta_ix = Transaction::new_signed_with_payer(
        &[create_token_acc_ix, initialize_token_account],
        Some(&context.payer.pubkey()),
        &[&context.payer, &token_account_key],
        context.last_blockhash,
    );

    let mint_ix = spl_token::instruction::mint_to(
        &spl_token::ID,
        &mint.pubkey(),
        &token_account_key.pubkey(),
        &context.payer.pubkey(),
        &[&context.payer.pubkey()],
        1,
    )
    .unwrap();

    let mint_tx = Transaction::new_signed_with_payer(
        &[mint_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    let tx = Transaction::new_signed_with_payer(
        &[create_account_ix, ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &mint],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await.unwrap();

    context
        .banks_client
        .process_transaction(create_ta_ix)
        .await
        .unwrap();

    context
        .banks_client
        .process_transaction(mint_tx)
        .await
        .unwrap();

    (mint.pubkey(), token_account_key.pubkey())
}
#[allow(dead_code)]
pub async fn create_and_verify_nft(
    context: &mut ProgramTestContext,
    nft_mint: &Pubkey,
    collection_key: Option<Pubkey>,
    send_transaction: bool,
) -> (Pubkey, Option<[Instruction; 2]>) {
    let metadata_account = find_metadata_account(nft_mint);

    let mut collection_details: Option<CollectionDetails> = None;
    let mut collection_data: Option<Collection> = None;

    if let Some(collection) = collection_key {
        collection_details = Some(CollectionDetails::V1 { size: 0 });
        collection_data = Some(Collection {
            verified: false,
            key: collection,
        });
    }

    let create_metadata_ix = mpl_token_metadata::instruction::create_metadata_accounts_v3(
        METADATA_PROGRAM,
        metadata_account.0,
        nft_mint.clone(),
        context.payer.pubkey(),
        context.payer.pubkey(),
        context.payer.pubkey(),
        "DeGod #0001".to_string(),
        "degods".to_string(),
        "some_uri".to_string(),
        Some(vec![Creator {
            address: context.payer.pubkey(),
            share: 100,
            verified: false,
        }]),
        500,
        true,
        false,
        collection_data,
        None,
        collection_details,
    );

    let edition = find_master_edition_account(nft_mint).0;

    let create_master_edition_ix = create_master_edition_v3(
        METADATA_PROGRAM,
        edition,
        nft_mint.clone(),
        context.payer.pubkey(),
        context.payer.pubkey(),
        metadata_account.0,
        context.payer.pubkey(),
        Some(0),
    );

    if send_transaction {
        let mut tx = Transaction::new_with_payer(
            &[create_metadata_ix, create_master_edition_ix],
            Some(&context.payer.pubkey()),
        );

        tx.sign(&[&context.payer], context.last_blockhash);
        context.banks_client.process_transaction(tx).await.unwrap();
    } else {
        return (
            metadata_account.0,
            Some([create_metadata_ix, create_master_edition_ix]),
        );
    }

    (metadata_account.0, None)
}
pub async fn fetch_nft_json(url: &str) -> UriMetadata {
    reqwest::get(url)
        .await
        .unwrap()
        .json::<UriMetadata>()
        .await
        .unwrap()
}

pub async fn deserialize_account_info<T>(
    context: &mut ProgramTestContext,
    account_key: &Pubkey,
) -> T
where
    T: BorshDeserialize + BorshSerialize,
{
    let account = context
        .banks_client
        .get_account(*account_key)
        .await
        .unwrap()
        .unwrap();

    try_from_slice_unchecked::<T>(&account.data.borrow()).unwrap()
}
