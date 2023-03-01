use solana_program::program_error::ProgramError;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum TraitError {
    #[error("Metadata is not related to provided collection")]
    InvalidCollection,
    #[error("Signer is not update authority of collection")]
    NotUpdateAuthority,
    #[error("Trait config not initialized")]
    TraitConfigNotInitialized,
    #[error("You don't have authority to store traits on chain!")]
    WrongAuthorityToCreateTrait,
    #[error("Trait does not exist in trait config")]
    TraitDoesNotExist,
}
impl From<TraitError> for ProgramError {
    fn from(e: TraitError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
