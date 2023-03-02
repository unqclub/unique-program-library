use solana_program_test::*;
mod nft;
pub use nft::*;
mod nft_state;
mod traits;
pub use traits::*;

pub fn program_test() -> ProgramTest {
    let mut program_test = ProgramTest::new("chain_traits", chain_traits::id(), None);

    program_test.add_program("mpl_token_metadata", mpl_token_metadata::id(), None);
    program_test.add_builtin_program(
        "spl_token",
        spl_token::id(),
        processor!(spl_token::processor::Processor::process).unwrap(),
    );

    program_test.add_program("mpl_token_metadata", mpl_token_metadata::id(), None);

    program_test
}
