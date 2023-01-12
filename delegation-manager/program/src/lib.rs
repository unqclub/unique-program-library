use anchor_lang::prelude::*;
declare_id!("5mcBrxdfAZZkBfThVY6HwkmZSbAhDNhNdxUiHqyhqZCA");

#[constant]
pub const AUTHORIZE_SEED: &'static [u8] = b"authorize";
/// Unique program library's Delegation Manager program.
#[program]
pub mod delegation_manager {
    use super::*;

    /// Initializes delegation account
    pub fn initialize_delegate(ctx: Context<InitializeDelegation>) -> Result<()> {
        let delegation = &mut ctx.accounts.delegation;
        delegation.master = ctx.accounts.master.key();
        delegation.representative = ctx.accounts.representative.key();
        delegation.authorised = false;
        Ok(())
    }

    /// Confirms delegation
    pub fn confirm_delegate(ctx: Context<ConfirmDelegation>) -> Result<()> {
        let delegation = &mut ctx.accounts.delegation;
        require!(
            ctx.accounts.representative.key() == delegation.representative,
            DelegationError::WrongRepresentative
        );
        require!(!delegation.authorised, DelegationError::AlreadyAuthorised);
        delegation.authorised = true;
        Ok(())
    }

    /// Cancels delegation
    pub fn cancel_delegate<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CancelDelegation<'info>>,
    ) -> Result<()> {
        let delegation = &mut ctx.accounts.delegation;
        let remaining_accounts = &mut ctx.remaining_accounts.iter();
        let master = remaining_accounts
            .next()
            .expect("Expected master as remaining account");
        let representative = remaining_accounts
            .next()
            .expect("Expected representative as remaining account");
        require!(
            master.key() == delegation.master,
            DelegationError::WrongMaster
        );
        require!(
            representative.key() == delegation.representative,
            DelegationError::WrongRepresentative
        );
        require!(
            master.is_signer || representative.is_signer,
            DelegationError::WrongSigner
        );

        delegation.close(master.to_account_info())?;

        Ok(())
    }
}

/// Accounts passed to InitializeDelegation instruction
#[derive(Accounts)]
pub struct InitializeDelegation<'info> {
    #[account(mut)]
    pub master: Signer<'info>,
    ///CHECK: can be any account which can sign confirmation
    pub representative: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [AUTHORIZE_SEED, master.key().as_ref(), representative.key().as_ref()],
        bump,
        space = 8 + 32 + 32 + 1,
        payer = master
    )]
    pub delegation: Box<Account<'info, Delegation>>,
    pub system_program: Program<'info, System>,
}

/// Accounts passed to ConfirmDelegation instruction
#[derive(Accounts)]
pub struct ConfirmDelegation<'info> {
    #[account(mut)]
    pub representative: Signer<'info>,
    #[account(mut)]
    pub delegation: Box<Account<'info, Delegation>>,
    pub system_program: Program<'info, System>,
}

/// Accounts passed to CancelDelegation instruction
#[derive(Accounts)]
pub struct CancelDelegation<'info> {
    #[account(mut)]
    pub delegation: Box<Account<'info, Delegation>>,
    pub system_program: Program<'info, System>,
}

/// State account storing the delegation
#[account]
pub struct Delegation {
    /// The creator of the delegation
    pub master: Pubkey,
    /// The wallet who delegates
    pub representative: Pubkey,
    /// Confirmation flag
    pub authorised: bool,
}

/// Program errors
#[error_code]
pub enum DelegationError {
    #[msg("Wrong representative!")]
    WrongRepresentative,
    #[msg("Wrong authority!")]
    WrongMaster,
    #[msg("Wrong signer!")]
    WrongSigner,
    #[msg("Authorization already approved!")]
    AlreadyAuthorised,
    #[msg("The account provided has no authority!")]
    NotAuthorized,
}

/// Function used to determine if a representative is authorised by master.
/// If the master is the same as a representative, the delegation_option argument can be None.
/// If the master is not the same as a representative, Delegation account needs to be passed
pub fn check_authorization(
    master: &AccountInfo,
    representative: &AccountInfo,
    delegation_option: Option<&AccountInfo>,
) -> Result<()> {
    if master.key() != representative.key() {
        let delegation_info = delegation_option.expect("Missing Delegation Account");
        require_keys_eq!(*delegation_info.owner, ID);
        let delegation = Box::new(
            Account::<Delegation>::try_from(delegation_info)
                .expect("Wrong account passed as Delegation account"),
        );
        require_keys_eq!(master.key(), delegation.master);
        require_keys_eq!(representative.key(), delegation.representative);
        require!(delegation.authorised, DelegationError::NotAuthorized);
    }
    Ok(())
}

pub fn get_delegation_address(master: &Pubkey, representative: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&get_delegation_address_seeds(master, representative), &ID).0
}

pub fn get_delegation_address_seeds<'a>(
    master: &'a Pubkey,
    representative: &'a Pubkey,
) -> [&'a [u8]; 3] {
    [AUTHORIZE_SEED, &master.as_ref(), &representative.as_ref()]
}
