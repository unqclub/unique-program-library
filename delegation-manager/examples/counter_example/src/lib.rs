use anchor_lang::prelude::*;

declare_id!("972QDtrTG4KvzEVt6fvxNmXQpuRyFhnpcR4Ln9Y41w5a");

/// This program shows an example of using the Unique Delegation Manager in a smart contract.
/// It contains a single instruction, 'increment_counter'. The first time it's invoked it creates
/// a Counter PDA account, and sets its authority to the one who signed the transaction. Each consecutive
/// time it's invoked, it checks if its invoked by the one who created the Counter account. If the signer
/// isn't the one who created it, it checks if the authoriti was delegated to the signer of the transaction,
/// so that he can increment the counter in the name of the one who created it. If the Delegation account
/// exists, the payer was authorised to represent the original authority of the Counter, an he has accepted
/// the Delegation, the counter is incremented.
#[program]
pub mod counter_example {
    use delegation_manager::check_authorization;

    use super::*;

    pub fn increment_counter(ctx: Context<IncrementCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        if counter.count == 0 {
            counter.authority = ctx.accounts.payer.key();
        } else {
            require_keys_eq!(ctx.accounts.authority.key(), counter.authority);
            check_authorization(
                &ctx.accounts.authority.to_account_info(),
                &ctx.accounts.payer.to_account_info(),
                ctx.remaining_accounts.iter().next(),
            )?;
        }
        counter.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct IncrementCounter<'info> {
    #[account(
        init_if_needed,
        seeds = [b"counter-state"],
        bump,
        payer = payer,
        space = 8 + 4 + 32,
    )]
    pub counter: Box<Account<'info, Counter>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    ///CHECK: Checked by check_authorization fn
    pub authority: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    count: u32,
    authority: Pubkey,
}
