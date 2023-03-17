use borsh::BorshSerialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
};

use crate::{instruction::CreateTraitConfigArgs, state::AvailableTrait};

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

pub fn shift_bytes(bytes: &mut [u8], new_data: &[u8], start_index: usize, new_array_len: u32) {
    msg!("NEW ARR LEN:{:?}", new_array_len);
    bytes[start_index..start_index + 4].copy_from_slice(&new_array_len.to_le_bytes());
    bytes.copy_within(
        start_index + 4..bytes.len() - new_data.len(),
        start_index + 4 + new_data.len(),
    );

    for (index, byte) in new_data.iter().enumerate() {
        bytes[index + start_index + 4] = *byte;
    }
}

pub fn add_new_traits_bytes(account_data: &mut [u8], data: Vec<CreateTraitConfigArgs>) {
    for arg in data.iter() {
        let serialized_arg_name = arg.name.try_to_vec().unwrap();
        let mut index = 76;

        loop {
            if index >= account_data.len() {
                break;
            }
            let key_length = get_u32_from_slice(&account_data[index..index + 4]) as usize;
            let key = &account_data[index..index + key_length + 4];

            let array_len_start = index + 4 + key_length;
            let array_length =
                get_u32_from_slice(&account_data[index + key_length + 4..index + key_length + 8])
                    as usize;

            let array_bytes =
                calculate_array_length(&account_data[index + key_length + 8..], array_length);

            if key == serialized_arg_name {
                let mapped_values: Vec<AvailableTrait> = arg
                    .values
                    .iter()
                    .map(|val| AvailableTrait {
                        is_active: true,
                        value: val.clone(),
                    })
                    .collect();
                let serialized_values = &mapped_values.try_to_vec().unwrap()[4..];

                shift_bytes(
                    account_data,
                    serialized_values,
                    array_len_start,
                    (array_length + arg.values.len()) as u32,
                );

                index += 4 + key_length + 4 + array_bytes + serialized_values.len();
            } else {
                index += 4 + key_length + 4 + array_bytes;
            }
        }
    }
}
