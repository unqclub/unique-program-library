use thiserror::Error;
#[derive(Error, Debug)]
pub enum TraitError {
    #[error("Metadata is not related to provided collection")]
    InvalidCollection,
    #[error("Signer is not update authority of collection")]
    NotUpdateAuthority,
}
