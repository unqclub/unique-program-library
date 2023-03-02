use std::str::FromStr;

use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

use crate::{
    state::{find_trait_config_address, find_trait_data_address},
    utils::SYSVAR_INSTRUCTIONS,
};

#[derive(BorshDeserialize, BorshSerialize, Debug, ShankInstruction)]
#[default_optional_accounts]
pub enum TraitInstruction {
    #[account(
        0,
        name = "collection",
        desc = "Collection or first creator of collection"
    )]
    #[account(
        1,
        name = "trait_config_account",
        desc = "Account used for storing trait config on-chain"
    )]
    #[account(
        2,
        name = "update_authority",
        signer,
        desc = "Signer of transaction(update authority of collection)"
    )]
    #[account(
        3,
        name = "collection_metadata",
        desc = "Metadata account of collection NFT or NFT from collection"
    )]
    #[account(4, name = "system_program")]
    CreateTraitConfig { data: Vec<CreateTraitConfigArgs> },

    #[account(0, name = "nft_mint", desc = "Mint of nft")]
    #[account(1, name = "nft_metadata", desc = "Metadata of nft")]
    #[account(
        2,
        name = "trait_config_account",
        desc = "Account used for storing trait config on-chain"
    )]
    #[account(
        3,
        name = "trait_account",
        desc = "Account used for storing traits for nft on-chain"
    )]
    #[account(
        4,
        name = "payer",
        desc = "Signer of transaction (update authority or holder in case of mint)",
        signer
    )]
    #[account(5, name = "system_program")]
    #[account(
        6,
        name = "instruction_sysvar",
        opt,
        desc = "Sysvar defining instruction of same transaction"
    )]
    CreateTrait { data: Vec<CreateTraitArgs> },
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct CreateTraitConfigArgs {
    pub name: String,
    pub values: Vec<String>,
    pub action: TraitAction,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub enum TraitAction {
    Add,
    Remove,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct CreateTraitArgs {
    pub name: String,
    pub value: String,
}

pub fn create_trait_config(
    program_id: &Pubkey,
    collection: &Pubkey,
    collection_metadata: &Pubkey,
    payer: &Pubkey,
    traits: Vec<CreateTraitConfigArgs>,
) -> Instruction {
    let (trait_config, _trait_config_bump) = find_trait_config_address(collection);

    let create_trait_accounts: Vec<AccountMeta> = vec![
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: *collection,
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: trait_config,
        },
        AccountMeta {
            is_signer: true,
            is_writable: false,
            pubkey: *payer,
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: *collection_metadata,
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: system_program::id(),
        },
    ];

    let data = TraitInstruction::CreateTraitConfig { data: traits }
        .try_to_vec()
        .unwrap();

    Instruction {
        program_id: *program_id,
        accounts: create_trait_accounts,
        data,
    }
}

pub fn create_trait(
    program_id: &Pubkey,
    mint: &Pubkey,
    mint_metadata: &Pubkey,
    trait_config: &Pubkey,
    payer: &Pubkey,
    traits: Vec<CreateTraitArgs>,
) -> Instruction {
    let (trait_account, _) = find_trait_data_address(trait_config, mint);

    let create_trait_accounts: Vec<AccountMeta> = vec![
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: *mint,
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: *mint_metadata,
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: *trait_config,
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: trait_account,
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: *payer,
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: system_program::id(),
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: Pubkey::from_str(SYSVAR_INSTRUCTIONS).unwrap(),
        },
    ];

    let data = TraitInstruction::CreateTrait { data: traits }
        .try_to_vec()
        .unwrap();

    Instruction {
        program_id: *program_id,
        accounts: create_trait_accounts,
        data,
    }
}
