use anchor_lang::prelude::*;

declare_id!("3Q8TuzBaXYjJtKyxAgZwr3ehkUWh2sBAwmwyjjJYHePK");

#[program]
pub mod delegate_manager {
    use super::*;

    pub fn initialize_delegate(ctx: Context<InitializeDelegate>) -> Result<()> {
        let representation = &mut ctx.accounts.representation;
        representation.master = ctx.accounts.master.key();
        representation.representative = ctx.accounts.representative.key();
        representation.authorised = false;
        Ok(())
    }

    pub fn confirm_delegate(ctx: Context<ConfirmDelegate>) -> Result<()> {
        let representation = &mut ctx.accounts.representation;
        require!(
            ctx.accounts.representative.key() == representation.representative,
            DMError::WrongRepresentative
        );
        require!(!representation.authorised, DMError::AlreadyAuthorised);
        representation.authorised = true;
        Ok(())
    }

    pub fn cancel_delegate<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CancelDelegate<'info>>,
    ) -> Result<()> {
        let representation = &mut ctx.accounts.representation;
        let remaining_accounts = &mut ctx.remaining_accounts.iter();
        let master = remaining_accounts
            .next()
            .expect("Expected master as remaining account");
        let representative = remaining_accounts
            .next()
            .expect("Expected representative as remaining account");
        require!(master.key() == representation.master, DMError::WrongMaster);
        require!(
            representative.key() == representation.representative,
            DMError::WrongRepresentative
        );
        require!(
            master.is_signer || representative.is_signer,
            DMError::WrongSigner
        );

        representation.close(master.to_account_info())?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeDelegate<'info> {
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
    pub representation: Box<Account<'info, Representation>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmDelegate<'info> {
    #[account(mut)]
    pub representative: Signer<'info>,
    #[account(mut)]
    pub representation: Box<Account<'info, Representation>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelDelegate<'info> {
    #[account(mut)]
    pub representation: Box<Account<'info, Representation>>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Representation {
    // The creator of the delegation
    pub master: Pubkey,
    // The wallet who delegates
    pub representative: Pubkey,
    // Confirmation flag
    pub authorised: bool,
}

#[error_code]
enum DMError {
    #[msg("Wrong delegator!")]
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

pub fn check_authorization(
    master: &AccountInfo,
    representative: &AccountInfo,
    representation: &Account<Representation>,
) -> Result<()> {
    require!(master.key() == representation.master, DMError::WrongMaster);
    require!(
        representative.key() == representation.representative,
        DMError::WrongRepresentative
    );
    require!(representation.authorised, DMError::NotAuthorized);

    Ok(())
}
