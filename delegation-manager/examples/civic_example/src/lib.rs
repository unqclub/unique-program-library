use anchor_lang::prelude::*;
use solana_gateway::Gateway;
use upl_delegation_manager::check_authorization;

declare_id!("9XMVrhZ7r6MxZ6fgHBL8rWbpRTAxtoK4EhvGBmjqj9ux");

#[constant]
pub const GATEKEEPER_NON_US: &str = "b1gz9sD7TeH6cagodm4zTcAx6LkZ56Etisvgr6jGFQb";
#[constant]
pub const GATEKEEPER_ALL: &str = "bni1ewus6aMxTxBi5SAfzEmmXLf8KcVFRmTfproJuKw";

#[program]
pub mod civic_example {
    use super::*;

    pub fn increment_counter(ctx: Context<IncrementCounter>, location: KycLocation) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        if counter.count == 0 {
            counter.authority = ctx.accounts.payer.key();
        } else {
            require_keys_eq!(ctx.accounts.authority.key(), counter.authority);

            let gateway_token = &ctx.accounts.gateway_token;

            let non_us = GATEKEEPER_NON_US.parse::<Pubkey>().unwrap();
            let all = GATEKEEPER_ALL.parse::<Pubkey>().unwrap();

            match location {
                KycLocation::OnlyUS => {
                    Gateway::verify_gateway_token_account_info(
                        gateway_token,
                        ctx.accounts.authority.key,
                        &all,
                        None,
                    )
                    .unwrap();
                }
                KycLocation::NonUS => {
                    Gateway::verify_gateway_token_account_info(
                        gateway_token,
                        ctx.accounts.authority.key,
                        &non_us,
                        None,
                    )
                    .unwrap();
                }
                KycLocation::All => {
                    let gatekeepers = vec![non_us, all];
                    for gatekeeper in gatekeepers {
                        let gateway_verification_result =
                            Gateway::verify_gateway_token_account_info(
                                gateway_token,
                                ctx.accounts.authority.key,
                                &gatekeeper,
                                None,
                            )
                            .map_err(|_| KycError::FailedVerification);
                        match gateway_verification_result {
                            Err(e) => return Err(error!(e)),
                            _ => {}
                        }
                    }
                }
            }

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
        seeds = [b"counter-state", authority.key().as_ref()],
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

    #[account()]
    ///CHECK
    pub gateway_token: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    count: u32,
    authority: Pubkey,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub enum KycLocation {
    All,
    OnlyUS,
    NonUS,
}

#[error_code]
pub enum KycError {
    #[msg("You do not pass the requirements")]
    FailedVerification,
}
