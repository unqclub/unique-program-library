use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
};

pub const SYSVAR_INSTRUCTIONS: &str = "Sysvar1nstructions1111111111111111111111111";

pub fn create_program_account<'a>(
    payer: &'a AccountInfo<'a>,
    new_account: &'a AccountInfo<'a>,
    signer_seeds: Option<&[&[u8]]>,
    owner_program: &Pubkey,
    space: u64,
    system_program: &'a AccountInfo<'a>,
) -> ProgramResult {
    let create_account_ix = system_instruction::create_account(
        payer.key,
        new_account.key,
        Rent::default().minimum_balance(space as usize),
        space,
        owner_program,
    );

    if let Some(signer_seeds) = signer_seeds {
        invoke_signed(
            &create_account_ix,
            &[payer.clone(), new_account.clone(), system_program.clone()],
            &[signer_seeds],
        )
    } else {
        invoke(
            &create_account_ix,
            &[payer.clone(), new_account.clone(), system_program.clone()],
        )
    }
}

pub fn transfer_lamports<'a>(
    source: &'a AccountInfo<'a>,
    destination: &'a AccountInfo<'a>,
    amount: u64,
    system_program: &'a AccountInfo<'a>,
) -> ProgramResult {
    let transfer_ix = system_instruction::transfer(source.key, destination.key, amount);

    invoke(
        &transfer_ix,
        &[source.clone(), destination.clone(), system_program.clone()],
    )
}
