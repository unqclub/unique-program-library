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

pub fn get_u32_from_slice(bytes: &[u8]) -> u32 {
    u32::from_le_bytes(bytes.try_into().unwrap())
}

pub fn calculate_array_length(bytes: &[u8], array_length: usize) -> usize {
    let mut arr_len: usize = 0;

    let mut index = 0;

    let mut bytes_indexer = 0;

    loop {
        if index >= array_length {
            break;
        }

        let arr_size = get_u32_from_slice(&bytes[bytes_indexer..bytes_indexer + 4]);

        arr_len += (4 + arr_size + 1) as usize;

        bytes_indexer += 4 + arr_size as usize + 1;
        index += 1;
    }

    arr_len
}

pub fn shift_bytes(bytes: &mut [u8], new_data: &[u8], mut start_index: usize, new_array_len: u32) {
    bytes[start_index..start_index + 4].copy_from_slice(&new_array_len.to_le_bytes());
    start_index += 4;
    bytes.copy_within(
        start_index..bytes.len() - new_data.len(),
        start_index + new_data.len(),
    );

    for (index, byte) in new_data.iter().enumerate() {
        bytes[index] = *byte;
    }
}
