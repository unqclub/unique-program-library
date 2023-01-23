# Unique Delegation Manager - UDM

An open-source, shared protocol for managing wallet delegation.

# About

Unique Delegation Manager is a toolset built for managing a "master-delegate" relationship between 1-to-many wallets. Protocols that implement it can allow safe execution of numerous actions for users without exposing their assets to any risks.

# Background

One of the core problems that the web3 industry is facing is a constant compromise between user experience and self-custody. While security is paramount, we cannot onboard the next billion users without UX that is human friendly. UDM was created with a goal to address this problem.

**UDM is open source and will always be open source. The goal is to also bring it as fast as possible to the point where the main program can be made immutable.**

UDM is a tool that allows users to create a "master-delegate" relationship between wallets. One master wallet can have multiple delegates, and one wallet can be a delegate to numerous master wallets.

**Delegates don't have, under any circumstances, an option to operate assets on the master account.**

From the user perspective, it's a simple interface that allows to manage such connections (create or delete). There is also a CLI version for advanced users, or an option to directly interact with the program using instructions.

From the app developer perspective, there are SDKs for programs and front-end that allow to validate the existence of master-delegate on-chain relationships and check master wallet assets.

That opens a number of use cases and possibilities, here are some examples:

1. If a user wallet is whitelisted for an NFT mint, he doesn't need to connect the main wallet to the minting site, but rather put funds for minting into a delegate wallet and only use that one.
2. If an app has claim functionality, it can allow delegates to claim reward, but send it to the master account.
3. Cross-device compatibility does not require copy-pasting the seed phrase or private keys any more. For instance, a mobile app can create a wallet for a new user via web3auth integration, and the user can add that new wallet as a delegate to his main wallet and have a frictionless mobile experience.

**User experience will become better with each app adopting the solution, so we encourage all developers to consider implementing it.**

# Program State

The UDM program utilizes one Program Derived Account:
* Delegation
## The Delegation Account

The delegation account is essentialy an on chain statement which confirms that the representative has the authority to execute smart contract actions that are otherwise reserved for the master. This account can also be used by projects to display asset ownership by proxy and give other logical priviledges.

```rust
#[account]
pub struct Delegation {
    /// The creator of the delegation
    pub master: Pubkey,
    /// The wallet who delegates
    pub representative: Pubkey,
    /// Confirmation flag
    pub authorised: bool,
}
```

The `master` field is the pubkey of the one who initiated delegation account. The `representative` field is the one who was invited to represent the master, and the `authorised` flag is set to **true** once the representative accepts the delegation.

# Integration

Once the Unique Delegation Manager platform, CLI or third party app was used to create the delegation, all that is required for projects to implement the UDM functionality is to add a single statement from the `delegation-manager` crate into their smart contract:

```rust
check_authorization(master_info, representative_info, delegation_info)?;
```

This function checks whether or not an account was authorised by master to represent it.

# Example usage

This program shows an example of using the Unique Delegation Manager in another Solana program. It contains a single instruction, 'increment_counter'. The first time it's invoked it creates a Counter PDA account, and sets its authority to the one who signed the transaction. Each consecutive time it's invoked, it checks if its invoked by the one who created the Counter account. If the signer isn't the one who created it, it checks if the authority was delegated to the signer of the transaction, so that he can increment the counter in the name of the one who created it. If the Delegation account exists, the payer was authorised to represent the original authority of the Counter, and he has accepted the Delegation, the counter is incremented.

```rust
#[program]
pub mod example {
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

```
