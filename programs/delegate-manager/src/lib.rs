use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod delegate_manager {
    use super::*;

    pub fn initialize_delegate(ctx: Context<InitializeDelegate>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.authority = ctx.accounts.authority.key();
        state.delegator = ctx.accounts.delegator.key();
        state.authorised = false;
        Ok(())
    }

    pub fn confirm_delegate(ctx: Context<ConfirmDelegate>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        require!(
            ctx.accounts.delegator.key() == state.delegator,
            DMError::WrongDelegator
        );
        require!(!state.authorised, DMError::AlreadyAuthorised);
        state.authorised = true;
        Ok(())
    }

    pub fn cancel_delegate<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CancelDelegate<'info>>,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let remaining_accounts = &mut ctx.remaining_accounts.iter();
        let authority = remaining_accounts
            .next()
            .expect("Expected authority as remaining account");
        let delegate = remaining_accounts
            .next()
            .expect("Expected delegate as remaining account");
        require!(authority.key() == state.authority, DMError::WrongAuthority);
        require!(delegate.key() == state.authority, DMError::WrongDelegator);
        require!(
            authority.is_signer || delegate.is_signer,
            DMError::WrongSigner
        );

        state.close(authority.to_account_info())?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeDelegate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    ///CHECK: can be any account who can sign confirmation
    pub delegator: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [b"authorize", authority.key().as_ref(), delegator.key().as_ref()],
        bump,
        space = 8 + 32 + 32 + 1,
        payer = authority
    )]
    pub state: Box<Account<'info, Delegation>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmDelegate<'info> {
    #[account(mut)]
    pub delegator: Signer<'info>,
    #[account(mut)]
    pub state: Box<Account<'info, Delegation>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelDelegate<'info> {
    #[account(mut)]
    pub state: Box<Account<'info, Delegation>>,
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
