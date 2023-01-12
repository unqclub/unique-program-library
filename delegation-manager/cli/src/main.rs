mod config;
use config::Config;

use clap::{
    crate_description, crate_name, crate_version, value_t, value_t_or_exit, App, AppSettings, Arg,
    ArgMatches, SubCommand,
};
use solana_clap_utils::{
    fee_payer::fee_payer_arg,
    input_parsers::{pubkey_of_signer, pubkeys_of_multiple_signers, value_of},
    input_validators::{
        is_amount, is_amount_or_all, is_parsable, is_pubkey, is_url_or_moniker, is_valid_pubkey,
        is_valid_signer,
    },
    keypair::signer_from_path,
    memo::memo_arg,
    nonce::*,
    offline::{self, *},
    ArgConstant,
};
use solana_remote_wallet::remote_wallet::RemoteWalletManager;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    signer, system_program,
    transaction::Transaction,
};
use std::{fmt::Display, process::exit, str::FromStr, sync::Arc};
use strum_macros::{EnumString, IntoStaticStr};

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Clone, Copy, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum CommandName {
    InitializeDelegation,
    ConfirmDelegation,
    CancelDelegation,
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
            SubCommand::with_name(CommandName::InitializeDelegation.into())
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
}

async fn command_initialize_delegation(
    config: &Config,
    signer: Arc<dyn Signer>,
    representative: Pubkey,
) {
    eprintln!(
        "InitializeDelegation Ix:\n-> Representative: {:#?} \n-> Signer: {:#?}\n-> ProgramId: {:#?}",
        representative,
        signer.pubkey(),
        config.program_id,
    );
    let ix = delegation_manager::instruction::InitializeDelegate;
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app_matches = app().get_matches();

    let (sub_command, sub_matches) = app_matches.subcommand();
    let sub_command = CommandName::from_str(sub_command).unwrap();
    let matches = sub_matches.unwrap();
    let mut wallet_manager = None;
    let config = Config::new(matches, &mut wallet_manager).await;

    process_command(&sub_command, matches, &config, wallet_manager).await;

    Ok(())
}

async fn process_command<'a>(
    sub_command: &CommandName,
    sub_matches: &ArgMatches<'_>,
    config: &Config,
    mut wallet_manager: Option<Arc<RemoteWalletManager>>,
) {
    match (sub_command, sub_matches) {
        (CommandName::InitializeDelegation, arg_matches) => {
            let recipient = pubkey_of_signer(arg_matches, "representative", &mut wallet_manager)
                .unwrap()
                .unwrap();
            let (owner_signer, _) =
                config.signer_or_default(arg_matches, "owner", &mut wallet_manager);

            command_initialize_delegation(config, owner_signer, recipient).await
        }
        _ => {
            todo!()
        }
    }
}
