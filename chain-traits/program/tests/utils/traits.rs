use std::borrow::Borrow;

use chain_traits::instruction::{
    create_trait, create_trait_config, CreateTraitArgs, CreateTraitConfigArgs,
};
use chain_traits::state::TraitConfig;
use mpl_token_metadata::instruction::Mint;
use solana_program::borsh::try_from_slice_unchecked;

use solana_program::instruction::Instruction;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction::create_account;
use solana_program_test::{BanksClientError, ProgramTestContext};
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use spl_token::instruction::initialize_account;
use spl_token::state::Account;

use super::{create_and_verify_nft, send_transaction};

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

pub async fn mint_and_store_trait(
    context: &mut ProgramTestContext,
    collection: &Pubkey,
    trait_config: &Pubkey,
    traits: Vec<CreateTraitArgs>,
    payer: &Keypair,
) -> Pubkey {
    let mint = Keypair::new();
    let create_account_ix = create_account(
        &context.payer.pubkey(),
        &mint.pubkey(),
        Rent::default().minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN as u64,
        &spl_token::ID,
    );

    let ix = spl_token::instruction::initialize_mint(
        &spl_token::ID,
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
        &spl_token::ID,
    );

    let initialize_token_account = initialize_account(
        &spl_token::ID,
        &token_account_key.pubkey(),
        &mint.pubkey(),
        &payer.pubkey(),
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

    // let mint_tx = Transaction::new_signed_with_payer(
    //     &[mint_ix],
    //     Some(&context.payer.pubkey()),
    //     &[&context.payer],
    //     context.last_blockhash,
    // );

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

    let (metadata, instructions) = create_and_verify_nft(
        context,
        &mint.pubkey(),
        Some(*collection),
        false,
        Some(payer),
    )
    .await;

    let mut tx_instructions: Vec<Instruction> = vec![mint_ix];

    instructions
        .unwrap()
        .iter()
        .for_each(|ix| tx_instructions.push(ix.clone()));

    let create_traits_ix = create_trait(
        &chain_traits::id(),
        &mint.pubkey(),
        &metadata,
        trait_config,
        &payer.pubkey(),
        traits,
    );

    tx_instructions.push(create_traits_ix);

    let trait_tx = Transaction::new_signed_with_payer(
        &tx_instructions,
        Some(&payer.pubkey()),
        &[&payer, &context.payer],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(trait_tx)
        .await
        .unwrap();

    mint.pubkey()
}
