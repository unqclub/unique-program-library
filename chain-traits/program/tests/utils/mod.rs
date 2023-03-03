use solana_program::{pubkey::Pubkey, system_instruction, system_program};
use solana_program_test::*;
mod nft;
pub use nft::*;
mod nft_state;
mod traits;
pub use nft_state::*;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
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

pub async fn airdrop_funds(context: &mut ProgramTestContext, destination: &Pubkey, amount: u64) {
    let transfer_ix = system_instruction::transfer(&context.payer.pubkey(), destination, amount);

    let tx = Transaction::new_signed_with_payer(
        &[transfer_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await.unwrap();
}

pub async fn create_account(context: &mut ProgramTestContext, new_account: &Keypair) {
    let create_acc_ix = system_instruction::create_account(
        &context.payer.pubkey(),
        &new_account.pubkey(),
        0,
        0,
        &system_program::id(),
    );

    let tx = Transaction::new_signed_with_payer(
        &[create_acc_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &new_account],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await.unwrap();
}
