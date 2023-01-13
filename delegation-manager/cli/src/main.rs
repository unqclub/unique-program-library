mod config;
use anchor_client::anchor_lang::{solana_program, AnchorSerialize, Discriminator};
use config::Config;

use clap::{
    crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches, SubCommand,
};

use delegation_manager::{get_delegation_address, Delegation};
use fastcmp::Compare;
use prettytable::ptable;
use solana_clap_utils::{
    fee_payer::fee_payer_arg,
    input_parsers::pubkey_of_signer,
    input_validators::{is_url_or_moniker, is_valid_pubkey},
};
use solana_remote_wallet::remote_wallet::RemoteWalletManager;
use solana_sdk::{
    borsh::try_from_slice_unchecked,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::Signer,
    system_program,
    transaction::Transaction,
};
use std::{str::FromStr, sync::Arc};
use strum_macros::{EnumString, IntoStaticStr};

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Clone, Copy, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum CommandName {
    Initialize,
    Confirm,
    Cancel,
    GetDelegations,
}

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("config_file")
                .short("C")
                .long("config")
                .value_name("PATH")
                .takes_value(true)
                .global(true)
                .help("Configuration file to use"),
        )
        .arg(
            Arg::with_name("json_rpc_url")
                .short("u")
                .long("url")
                .value_name("URL_OR_MONIKER")
                .takes_value(true)
                .global(true)
                .validator(is_url_or_moniker)
                .help(
                    "URL for Solana's JSON RPC or moniker (or their first letter): \
                       [mainnet-beta, testnet, devnet, localhost] \
                    Default from the configuration file.",
                ),
        )
        .arg(fee_payer_arg().global(true))
        .subcommand(
            SubCommand::with_name(CommandName::Initialize.into())
                .about("Initialize Delegation")
                .arg(
                    Arg::with_name("representative")
                        .value_name("REPRESENTATIVE")
                        .validator(is_valid_pubkey)
                        .takes_value(true)
                        .index(1)
                        .help(
                            "Specify the delegation representative. \
                            This must be a valid public key.",
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name(CommandName::Confirm.into())
                .about("Confirm Delegation")
                .arg(
                    Arg::with_name("delegation")
                        .value_name("DELEGATION")
                        .validator(is_valid_pubkey)
                        .takes_value(true)
                        .index(1)
                        .help(
                            "Specify the delegation to confirm. \
                            This must be a valid public key.",
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name(CommandName::Cancel.into())
                .about("Cancel Delegation")
                .arg(
                    Arg::with_name("delegation")
                        .value_name("DELEGATION")
                        .validator(is_valid_pubkey)
                        .takes_value(true)
                        .index(1)
                        .help(
                            "Specify the delegation to cancel. \
                            This must be a valid public key.",
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name(CommandName::GetDelegations.into())
                .about("Get Delegations")
                .arg(
                    Arg::with_name("delegation")
                        .value_name("DELEGATION")
                        .validator(is_valid_pubkey)
                        .takes_value(true)
                        .index(1)
                        .help(
                            "Displays delegation for a given master. \
                            This must be a valid public key.",
                        ),
                ),
        )
}

async fn command_initialize_delegation(
    config: &Config,
    signer: Arc<dyn Signer>,
    representative: Pubkey,
) -> Result<(), Error> {
    let instruction = Instruction {
        accounts: vec![
            AccountMeta::new(signer.pubkey(), true),
            AccountMeta::new_readonly(representative, false),
            AccountMeta::new(
                get_delegation_address(&signer.pubkey(), &representative),
                false,
            ),
            AccountMeta::new(system_program::ID, false),
        ],
        program_id: config.program_id.clone(),
        data: sighash("global", "initialize_delegate")
            .try_to_vec()
            .unwrap(),
    };

    let message = Message::new_with_blockhash(
        &[instruction],
        Some(&signer.pubkey()),
        &config.rpc_client.get_latest_blockhash().await.unwrap(),
    );
    let signature = signer.sign_message(&message.serialize());
    let mut transaction = Transaction::new_unsigned(message);
    transaction
        .replace_signatures(&[(signer.pubkey(), signature)])
        .unwrap();

    config
        .rpc_client
        .send_and_confirm_transaction(&transaction)
        .await
        .unwrap();

    Ok(())
}

async fn command_get_delegations(config: &Config, pubkey: &Pubkey) -> Result<(), Error> {
    let delegation_accounts = config
        .rpc_client
        .get_program_accounts(&delegation_manager::ID)
        .await?;

    let mut master_delegations = Vec::new();
    let mut representative_delegations = Vec::new();

    let parsed_accounts = delegation_accounts
        .iter()
        .filter(|(_, account)| account.data[0..8].feq(&Delegation::discriminator()))
        .map(|(address, account)| {
            let account = try_from_slice_unchecked::<Delegation>(&account.data[8..]).unwrap();
            if &account.master == pubkey {
                master_delegations.push(account.clone());
            } else if &account.representative == pubkey {
                representative_delegations.push(account.clone());
            }
            account
        })
        .filter(|account| account.master == pubkey.clone())
        .collect::<Vec<_>>();

    eprint!("{:#?}", parsed_accounts);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app_matches = app().get_matches();

    let (sub_command, sub_matches) = app_matches.subcommand();
    let sub_command = CommandName::from_str(sub_command).unwrap();
    let matches = sub_matches.unwrap();
    let mut wallet_manager = None;
    let config = Config::new(matches, &mut wallet_manager).await;

    process_command(&sub_command, matches, &config, wallet_manager).await?;

    Ok(())
}

async fn process_command<'a>(
    sub_command: &CommandName,
    sub_matches: &ArgMatches<'_>,
    config: &Config,
    mut wallet_manager: Option<Arc<RemoteWalletManager>>,
) -> Result<(), Error> {
    match (sub_command, sub_matches) {
        (CommandName::Initialize, arg_matches) => {
            let recipient = pubkey_of_signer(arg_matches, "representative", &mut wallet_manager)
                .unwrap()
                .unwrap();
            let (owner_signer, _) =
                config.signer_or_default(arg_matches, "owner", &mut wallet_manager);

            command_initialize_delegation(config, owner_signer, recipient).await?;
            Ok(())
        }
        (CommandName::Confirm, arg_matches) => {
            let (_owner_signer, _) =
                config.signer_or_default(arg_matches, "owner", &mut wallet_manager);
            todo!()
        }
        (CommandName::Cancel, arg_matches) => {
            let (_owner_signer, _) =
                config.signer_or_default(arg_matches, "owner", &mut wallet_manager);
            todo!()
        }
        (CommandName::GetDelegations, arg_matches) => {
            let pubkey = config.pubkey_or_default(arg_matches, "owner", &mut wallet_manager)?;
            command_get_delegations(config, &pubkey).await?;
            Ok(())
        }
    }
}

pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);

    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(&solana_program::hash::hash(preimage.as_bytes()).to_bytes()[..8]);
    sighash
}
