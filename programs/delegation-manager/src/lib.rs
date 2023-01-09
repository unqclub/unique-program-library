use anchor_lang::prelude::*;

declare_id!("3Q8TuzBaXYjJtKyxAgZwr3ehkUWh2sBAwmwyjjJYHePK");

#[program]
pub mod delegate_manager {
    use super::*;

    pub fn initialize_delegate(ctx: Context<InitializeDelegation>) -> Result<()> {
        let delegation = &mut ctx.accounts.delegation;
        delegation.master = ctx.accounts.master.key();
        delegation.representative = ctx.accounts.representative.key();
        delegation.authorised = false;
        Ok(())
    }

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

#[derive(Accounts)]
pub struct InitializeDelegation<'info> {
    #[account(mut)]
    pub master: Signer<'info>,
    ///CHECK: can be any account which can sign confirmation
    pub representative: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [b"authorize", master.key().as_ref(), representative.key().as_ref()],
        bump,
        space = 8 + 32 + 32 + 1,
        payer = master
    )]
    pub delegation: Box<Account<'info, Delegation>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmDelegation<'info> {
    #[account(mut)]
    pub representative: Signer<'info>,
    #[account(mut)]
    pub delegation: Box<Account<'info, Delegation>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelDelegation<'info> {
    #[account(mut)]
    pub delegation: Box<Account<'info, Delegation>>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Delegation {
    // The creator of the delegation
    pub master: Pubkey,
    // The wallet who delegates
    pub representative: Pubkey,
    // Confirmation flag
    pub authorised: bool,
}

#[error_code]
enum DelegationError {
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

pub fn check_authorization(representative: &AccountInfo, delegation: &AccountInfo) -> Result<()> {
    let delegation = Box::new(
        Account::<Delegation>::try_from(delegation)
            .expect("Wrong account passed as Delegation account"),
    );
    require_keys_eq!(representative.key(), delegation.representative);
    require!(delegation.authorised, DelegationError::NotAuthorized);

    Ok(())
}
