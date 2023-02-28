use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[derive(BorshDeserialize, BorshSerialize, Debug, ShankInstruction)]
pub enum TraitInstruction {
    #[account(0, name = "First account", desc = "")]
    CreateTraitConfig {},
}
