//! Error types

use {
    num_derive::FromPrimitive,
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

/// Errors that may be returned by the Token program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum DelegationError {
    #[error("Wrong representative!")]
    WrongRepresentative,
    #[error("Wrong authority!")]
    WrongMaster,
    #[error("Wrong signer!")]
    WrongSigner,
    #[error("Authorization already approved!")]
    AlreadyAuthorised,
    #[error("The account provided has no authority!")]
    NotAuthorized,
    #[error("Wrong delegation account for given master and representative!")]
    WrongDelegation,
}
impl From<DelegationError> for ProgramError {
    fn from(e: DelegationError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for DelegationError {
    fn type_of() -> &'static str {
        "DelegationError"
    }
}

impl PrintProgramError for DelegationError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + num_traits::FromPrimitive,
    {
        match self {
            DelegationError::WrongRepresentative => {
                msg!("Wrong representative!")
            }
            DelegationError::WrongMaster => {
                msg!("Wrong authority!")
            }
            DelegationError::WrongSigner => {
                msg!("Wrong signer!")
            }
            DelegationError::AlreadyAuthorised => {
                msg!("Authorization already approved!")
            }
            DelegationError::NotAuthorized => {
                msg!("The account provided has no authority!")
            }
            DelegationError::WrongDelegation => {
                msg!("Wrong delegation account for given master and representative!")
            }
        }
    }
}
