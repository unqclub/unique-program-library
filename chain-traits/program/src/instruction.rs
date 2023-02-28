use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

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
    CreateTraitConfig {
        data: Vec<CreateTraitConfigArgs>,
    },

    #[account(0, name="nft_mint", desc="Mint of nft")]
    #[account(1, name="nft_metadata", desc="Metadata of nft")]
    #[account(2, name="trait_config_account", desc="Account used for storing trait config on-chain")]
    #[account(3, name="trait_account", desc="Account used for storing traits for nft on-chain")]
    #[account(4, name="payer", desc="Signer of transaction (update authority or holder in case of mint)", signer)]
    #[account(5, name="system_program")]
    #[account(6, name="instruction_sysvar"), opt, desc="Sysvar defining instruction of same transaction"]
    CreateTrait {
        data: Vec<CreateTraitArgs>,
    },
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
    pub name: String;
    pub value: String;
}