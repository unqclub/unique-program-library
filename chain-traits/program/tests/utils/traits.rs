use std::borrow::Borrow;

use chain_traits::instruction::{
    create_trait, create_trait_config, CreateTraitArgs, CreateTraitConfigArgs,
};
use chain_traits::state::TraitConfig;
use solana_program::borsh::try_from_slice_unchecked;
use solana_program::instruction::Instruction;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program_test::{BanksClientError, ProgramTestContext};
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

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
) {
    let token = Keypair::new();
    let mint = Keypair::new();

    let create_mint_account = system_instruction::create_account(
        &payer.pubkey(),
        &token.pubkey(),
        Rent::default().minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN as u64,
        &spl_token::ID,
    );

    let init_mint = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &mint.pubkey(),
        &payer.pubkey(),
        Some(&context.payer.pubkey()),
        0,
    )
    .unwrap();

    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &token.pubkey(),
        Rent::default().minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN as u64,
        &spl_token::ID,
    );

    let init_token_account = spl_token::instruction::initialize_account(
        &spl_token::id(),
        &token.pubkey(),
        &mint.pubkey(),
        &payer.pubkey(),
    )
    .unwrap();
    send_transaction(
        context,
        &[
            create_mint_account,
            init_mint,
            create_account_ix,
            init_token_account,
        ],
        None,
        None,
    )
    .await;

    let mint_to_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint.pubkey(),
        &token.pubkey(),
        &payer.pubkey(),
        &[&payer.pubkey()],
        1,
    )
    .unwrap();

    let (metadata, instructions) =
        create_and_verify_nft(context, &mint.pubkey(), Some(*collection), false).await;

    let mut ix: Vec<Instruction> = vec![mint_to_ix];

    instructions
        .unwrap()
        .iter()
        .for_each(|ix_data| ix.push(ix_data.clone()));

    let store_trait_ix = create_trait(
        &chain_traits::id(),
        &mint.pubkey(),
        &metadata,
        trait_config,
        &payer.pubkey(),
        traits,
    );

    ix.push(store_trait_ix);

    send_transaction(context, &ix, Some(payer), None).await;
}
