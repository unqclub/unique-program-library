use anchor_lang::prelude::*;

declare_id!("972QDtrTG4KvzEVt6fvxNmXQpuRyFhnpcR4Ln9Y41w5a");

#[program]
pub mod example {
    use delegation_manager::check_authorization;

    use super::*;

    pub fn increment_counter(ctx: Context<IncrementCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        if counter.count == 0 {
            counter.authority = ctx.accounts.payer.key();
        } else {
            if counter.authority != ctx.accounts.payer.key() {
                let delegation = ctx
                    .remaining_accounts
                    .iter()
                    .next()
                    .expect("Missing Delegation account");
                check_authorization(&ctx.accounts.payer.to_account_info(), delegation)?;
            }
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
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    count: u32,
    authority: Pubkey,
}
