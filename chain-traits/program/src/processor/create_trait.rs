use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar::instructions::{load_current_index_checked, load_instruction_at_checked},
};
use spl_token::state::Mint;

use crate::{errors::TraitError, instruction::CreateTraitArgs, state::TraitConfig};

pub fn create_trait<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: Vec<CreateTraitArgs>,
) -> ProgramResult {
    let account_infos = &mut accounts.iter();
    let nft_mint_info = next_account_info(account_infos)?;
    let nft_metadata_info = next_account_info(account_infos)?;
    let trait_config_account_info = next_account_info(account_infos)?;
    let trait_account_info = next_account_info(account_infos)?;
    let payer = next_account_info(account_infos)?;
    let system_program = next_account_info(account_infos)?;
    let instructon_sysvar = next_account_info(account_infos)?;

    let instruction_index = load_current_index_checked(instructon_sysvar)?;

    assert!(
        !trait_account_info.data_is_empty(),
        "{}",
        TraitError::TraitConfigNotInitialized
    );

    let mint_acc = Mint::unpack(&nft_mint_info.data.borrow())?;

    let trait_config =
        try_from_slice_unchecked::<TraitConfig>(&trait_config_account_info.data.borrow())?;

    if *payer.key != trait_config.update_authoirty {
        let mut mint_to_executed = false;
        for i in 0..instruction_index {
            let instruction = load_instruction_at_checked(i.into(), instructon_sysvar)?;
            if instruction.program_id == spl_token::id()
                && instruction.accounts.get(0).unwrap().pubkey == *nft_mint_info.key
            {
                mint_to_executed = true;
                break;
            }
        }

        assert!(
            mint_to_executed,
            "{}",
            TraitError::WrongAuthorityToCreateTrait
        );
    }

    Ok(())
}
