
pub fn create_trait<'a>(program_id: &Pubkey, accounts: &'a [AccountInfo[<'a>]])->ProgramResult {
    let account_infos = &mut accounts.iter();
    let nft_mint_info = account_infos.next()?;
    let nft_metadata_info = account_info.next()?;
    let trait_config_account_info = account_info.next()?;
    let trait_account_info = account_info.next()?;
}