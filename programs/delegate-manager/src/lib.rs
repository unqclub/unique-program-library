use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod delegate_manager {
    use super::*;

    pub fn initialize_delegate(ctx: Context<InitializeDelegate>) -> Result<()> {
        let delegation = &mut ctx.accounts.delegation;
        delegation.authority = ctx.accounts.authority.key();
        delegation.delegator = ctx.accounts.delegator.key();
        delegation.authorised = false;
        Ok(())
    }

    pub fn confirm_delegate(ctx: Context<ConfirmDelegate>) -> Result<()> {
        let delegation = &mut ctx.accounts.delegation;
        require!(
            ctx.accounts.delegator.key() == delegation.delegator,
            DMError::WrongDelegator
        );
        require!(!delegation.authorised, DMError::AlreadyAuthorised);
        delegation.authorised = true;
        Ok(())
    }

    pub fn cancel_delegate<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CancelDelegate<'info>>,
    ) -> Result<()> {
        let delegation = &mut ctx.accounts.delegation;
        let remaining_accounts = &mut ctx.remaining_accounts.iter();
        let authority = remaining_accounts
            .next()
            .expect("Expected authority as remaining account");
        let delegate = remaining_accounts
            .next()
            .expect("Expected delegate as remaining account");
        require!(
            authority.key() == delegation.authority,
            DMError::WrongAuthority
        );
        require!(
            delegate.key() == delegation.delegator,
            DMError::WrongDelegator
        );
        require!(
            authority.is_signer || delegate.is_signer,
            DMError::WrongSigner
        );

        delegation.close(authority.to_account_info())?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeDelegate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    ///CHECK: can be any account which can sign confirmation
    pub delegator: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [b"authorize", authority.key().as_ref(), delegator.key().as_ref()],
        bump,
        space = 8 + 32 + 32 + 1,
        payer = authority
    )]
    pub delegation: Box<Account<'info, Delegation>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmDelegate<'info> {
    #[account(mut)]
    pub delegator: Signer<'info>,
    #[account(mut)]
    pub delegation: Box<Account<'info, Delegation>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelDelegate<'info> {
    #[account(mut)]
    pub delegation: Box<Account<'info, Delegation>>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Delegation {
    // The creator of the delegation
    pub authority: Pubkey,
    // The wallet who delegates
    pub delegator: Pubkey,
    // Confirmation flag
    pub authorised: bool,
}

#[error_code]
enum DMError {
    #[msg("Wrong delegator!")]
    WrongDelegator,
    #[msg("Wrong authority!")]
    WrongAuthority,
    #[msg("Wrong signer!")]
    WrongSigner,
    #[msg("Authorization already approved!")]
    AlreadyAuthorised,
}
