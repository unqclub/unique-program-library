use crate::Error;
use clap::ArgMatches;
use solana_clap_utils::{
    input_parsers::{pubkey_of_signer, value_of},
    input_validators::normalize_to_url_if_moniker,
    keypair::{signer_from_path, signer_from_path_with_config, SignerFromPathConfig},
    offline::{DUMP_TRANSACTION_MESSAGE, SIGN_ONLY_ARG},
};
use solana_cli_output::OutputFormat;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_remote_wallet::remote_wallet::RemoteWalletManager;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signer};
use std::{process::exit, sync::Arc};

#[allow(unused)]
pub(crate) struct Config {
    pub(crate) default_signer: Option<Arc<dyn Signer>>,
    pub(crate) rpc_client: Arc<RpcClient>,
    pub(crate) websocket_url: String,
    pub(crate) output_format: OutputFormat,
    pub(crate) fee_payer: Option<Arc<dyn Signer>>,
    pub(crate) sign_only: bool,
    pub(crate) dump_transaction_message: bool,
    pub(crate) program_id: Pubkey,
    pub(crate) restrict_to_program_id: bool,
}

#[allow(unused)]
impl Config {
    pub(crate) async fn new(
        matches: &ArgMatches<'_>,
        wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
    ) -> Config {
        let cli_config = if let Some(config_file) = matches.value_of("config_file") {
            solana_cli_config::Config::load(config_file).unwrap_or_else(|_| {
                eprintln!("error: Could not find config file `{}`", config_file);
                exit(1);
            })
        } else if let Some(config_file) = &*solana_cli_config::CONFIG_FILE {
            solana_cli_config::Config::load(config_file).unwrap_or_default()
        } else {
            solana_cli_config::Config::default()
        };
        let json_rpc_url = normalize_to_url_if_moniker(
            matches
                .value_of("json_rpc_url")
                .unwrap_or(&cli_config.json_rpc_url),
        );
        let websocket_url = solana_cli_config::Config::compute_websocket_url(&json_rpc_url);
        let rpc_client = Arc::new(RpcClient::new_with_commitment(
            json_rpc_url,
            CommitmentConfig::confirmed(),
        ));

        Self::new_with_clients_and_ws_url(matches, wallet_manager, rpc_client, websocket_url).await
    }

    pub(crate) async fn new_with_clients_and_ws_url(
        matches: &ArgMatches<'_>,
        wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
        rpc_client: Arc<RpcClient>,
        websocket_url: String,
    ) -> Config {
        let cli_config = if let Some(config_file) = matches.value_of("config_file") {
            solana_cli_config::Config::load(config_file).unwrap_or_else(|_| {
                eprintln!("error: Could not find config file `{}`", config_file);
                exit(1);
            })
        } else if let Some(config_file) = &*solana_cli_config::CONFIG_FILE {
            solana_cli_config::Config::load(config_file).unwrap_or_default()
        } else {
            solana_cli_config::Config::default()
        };

        let default_keypair = cli_config.keypair_path.clone();

        let config = SignerFromPathConfig {
            allow_null_signer: true,
        };

        let default_signer: Option<Arc<dyn Signer>> = {
            if let Some(owner_path) = matches.value_of("owner") {
                signer_from_path_with_config(matches, owner_path, "owner", wallet_manager, &config)
                    .ok()
            } else {
                signer_from_path_with_config(
                    matches,
                    &default_keypair,
                    "default",
                    wallet_manager,
                    &config,
                )
                .map_err(|e| {
                    if std::fs::metadata(&default_keypair).is_ok() {
                        eprintln!("error: {}", e);
                        exit(1);
                    } else {
                        e
                    }
                })
                .ok()
            }
        }
        .map(Arc::from);

        let fee_payer: Option<Arc<dyn Signer>> = matches
            .value_of("fee_payer")
            .map(|path| {
                Arc::from(
                    signer_from_path(matches, path, "fee_payer", wallet_manager).unwrap_or_else(
                        |e| {
                            eprintln!("error: {}", e);
                            exit(1);
                        },
                    ),
                )
            })
            .or_else(|| default_signer.clone());

        let verbose = matches.is_present("verbose");
        let output_format = matches
            .value_of("output_format")
            .map(|value| match value {
                "json" => OutputFormat::Json,
                "json-compact" => OutputFormat::JsonCompact,
                _ => unreachable!(),
            })
            .unwrap_or(if verbose {
                OutputFormat::DisplayVerbose
            } else {
                OutputFormat::Display
            });

        let sign_only = matches.is_present(SIGN_ONLY_ARG.name);
        let dump_transaction_message = matches.is_present(DUMP_TRANSACTION_MESSAGE.name);

        let default_program_id = upl_delegation_manager::ID;
        let (program_id, restrict_to_program_id) =
            if let Some(program_id) = value_of(matches, "program_id") {
                (program_id, true)
            } else if !sign_only {
                if let Some(address) = value_of(matches, "token")
                    .or_else(|| value_of(matches, "account"))
                    .or_else(|| value_of(matches, "address"))
                {
                    (
                        rpc_client
                            .get_account(&address)
                            .await
                            .map(|account| account.owner)
                            .unwrap_or(default_program_id),
                        false,
                    )
                } else {
                    (default_program_id, false)
                }
            } else {
                (default_program_id, false)
            };

        Self {
            default_signer,
            rpc_client,
            websocket_url,
            output_format,
            fee_payer,
            sign_only,
            dump_transaction_message,
            program_id,
            restrict_to_program_id,
        }
    }

    // Returns Ok(default signer), or Err if there is no default signer configured
    pub(crate) fn default_signer(&self) -> Result<Arc<dyn Signer>, Error> {
        if let Some(default_signer) = &self.default_signer {
            Ok(default_signer.clone())
        } else {
            Err("default signer is required, please specify a valid default signer by identifying a \
                 valid configuration file using the --config-file argument, or by creating a valid \
                 config at the default location of ~/.config/solana/cli/config.yml using the solana \
                 config command".to_string().into())
        }
    }

    // Returns Ok(fee payer), or Err if there is no fee payer configured
    pub(crate) fn fee_payer(&self) -> Result<Arc<dyn Signer>, Error> {
        if let Some(fee_payer) = &self.fee_payer {
            Ok(fee_payer.clone())
        } else {
            Err("fee payer is required, please specify a valid fee payer using the --fee_payer argument, \
                 or by identifying a valid configuration file using the --config-file argument, or by \
                 creating a valid config at the default location of ~/.config/solana/cli/config.yml using \
                 the solana config command".to_string().into())
        }
    }

    // Checks if an explicit address was provided, otherwise return the default address if there is one
    pub(crate) fn pubkey_or_default(
        &self,
        arg_matches: &ArgMatches<'_>,
        address_name: &str,
        wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
    ) -> Result<Pubkey, Error> {
        if let Some(address) = pubkey_of_signer(arg_matches, address_name, wallet_manager).unwrap()
        {
            return Ok(address);
        }

        Ok(self.default_signer()?.pubkey())
    }

    // Checks if an explicit signer was provided, otherwise return the default signer.
    pub(crate) fn signer_or_default(
        &self,
        arg_matches: &ArgMatches,
        authority_name: &str,
        wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
    ) -> (Arc<dyn Signer>, Pubkey) {
        // If there are `--multisig-signers` on the command line, allow `NullSigner`s to
        // be returned for multisig account addresses
        let config = SignerFromPathConfig {
            allow_null_signer: true,
        };
        let mut load_authority = move || -> Result<Arc<dyn Signer>, Error> {
            if authority_name != "owner" {
                if let Some(keypair_path) = arg_matches.value_of(authority_name) {
                    return signer_from_path_with_config(
                        arg_matches,
                        keypair_path,
                        authority_name,
                        wallet_manager,
                        &config,
                    )
                    .map(Arc::from)
                    .map_err(|e| e.to_string().into());
                }
            }

            self.default_signer()
        };
        let authority = load_authority().unwrap_or_else(|e| {
            eprintln!("error: {}", e);
            exit(1);
        });

        let authority_address = authority.pubkey();
        (authority, authority_address)
    }
}
