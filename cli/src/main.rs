use std::io::Read;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

use api_v1::date_time::DateTime;
use api_v1::gateway::GatewayMetric;
use client::client::Client as VcoClient;

mod keyring;
mod property;

fn fqdn_to_name_and_domain(vco_fqdn: &str) -> Result<(String, String)> {
    let parts = vco_fqdn.splitn(2, ".").collect::<Vec<&str>>();
    if parts.len() != 2 {
        return Err(anyhow::format_err!(
            "Bad FQDN format, expected at least one dot in name, not \"{vco_fqdn}\"."
        ));
    }
    let vco_name = parts[0].to_string().to_lowercase();
    let vco_domain = parts[1].to_string().to_lowercase();
    if !vco_name.starts_with("vco") {
        return Err(anyhow::format_err!(
            "VCO name must start with \"vco\"; got \"{vco_fqdn}\"."
        ));
    }
    Ok((vco_name, vco_domain))
}

/// Build a `VcoClient` given the VCO's FQDN and credentials.
async fn client_from_creds(vco_fqdn: &str, creds_source: &CredentialSource) -> Result<VcoClient> {
    let (vco_name, vco_domain) = fqdn_to_name_and_domain(&vco_fqdn)?;
    let vco = if creds_source.is_token() {
        let (_, token) = creds_source.acquire(&vco_fqdn)?;
        VcoClient::operator_login_token(&vco_name, &vco_domain, &token)
            .await
            .map_err(|_| {
                anyhow::format_err!("Could not log into {vco_fqdn} with the given token.")
            })?
    } else {
        let (username, password) = creds_source.acquire(&vco_fqdn)?;
        VcoClient::operator_login_password(&vco_name, &vco_domain, &username, &password)
            .await
            .map_err(|_| {
                anyhow::format_err!(
                    "Could not log into {vco_fqdn} as {username} with the given password."
                )
            })?
    };
    Ok(vco)
}

fn is_email(value: &str) -> Result<String, String> {
    let parts = value.split("@").collect::<Vec<&str>>();
    if parts.len() != 2 || !parts[1].contains(".") {
        Err(format!("Value is not a valid email: {value}"))
    } else {
        Ok(value.to_string())
    }
}

/// Base of the CLI command tree.
#[derive(Debug, Parser)]
#[command(name = "vcoctl")]
#[command(about = "CLI tool for interacting with VMware SD-WAN Orchestrator")]
struct Cli {
    vco_fqdn: String,

    #[command(subcommand)]
    command: Commands,
}

/// Arguments for the source of VCO credentials.
#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct CredentialSource {
    /// Prompt for the password of this user.
    #[arg(long, value_name = "USERNAME")]
    prompt: Option<String>,

    /// Prompt for an API token.
    #[arg(long)]
    token: bool,

    /// Read a token from the keyring for this user.
    #[arg(long, value_name = "USERNAME")]
    keyring_token: Option<String>,

    /// Read a password from the keyring for this user.
    #[arg(long, value_name = "USERNAME")]
    keyring_password: Option<String>,
}

impl CredentialSource {
    /// Fetch the credential according to the option presented.
    fn acquire(&self, vco_fqdn: &str) -> Result<(String, String)> {
        if let Some(username) = &self.prompt {
            Ok((
                username.to_string(),
                rpassword::prompt_password(&format!("Password for {username} on {vco_fqdn}: "))?,
            ))
        } else if self.token {
            Ok((
                String::new(),
                rpassword::prompt_password(&format!("API token for {vco_fqdn}: "))?,
            ))
        } else if let Some(username) = &self.keyring_token {
            Ok((
                username.to_string(),
                keyring::get_token(&vco_fqdn, &username)?,
            ))
        } else if let Some(username) = &self.keyring_password {
            Ok((
                username.to_string(),
                keyring::get_password(&vco_fqdn, &username)?,
            ))
        } else {
            unreachable!()
        }
    }

    fn is_token(&self) -> bool {
        self.token || self.keyring_token.is_some()
    }
}

/// Top-level CLI commands.
#[derive(Debug, Subcommand)]
enum Commands {
    /// Actions on VCO credentials in the system Keyring.
    Keyring {
        /// The operator username.
        #[arg(value_parser(is_email))]
        username: String,

        #[command(subcommand)]
        action: KeyringCommand,
    },

    /// Actions on VCO system properties
    Property {
        #[command(flatten)]
        creds_source: CredentialSource,

        #[command(subcommand)]
        action: PropertyCommand,
    },

    GatewayMetric {
        #[command(flatten)]
        creds_source: CredentialSource,
    },
}

/// Keyring commands.
#[derive(Debug, Subcommand)]
enum KeyringCommand {
    /// Set an operator password for the VCO.
    SetPassword,

    /// Set an operator token for the VCO.
    SetToken,

    /// Delete an operator password for the VCO.
    DeletePassword,

    /// Delete an operator token for the VCO.
    DeleteToken,
}

/// VCO System Property commands
#[derive(Debug, Subcommand)]
enum PropertyCommand {
    List {
        #[arg(long, required = false, default_value = "")]
        filter: String,

        #[arg(long, required = false, default_value = "false")]
        show_passwords: bool,
    },
    Get {
        name: String,
    },
    Set,
    Delete,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let vco_fqdn = args.vco_fqdn;

    let output_message: String = match args.command {
        Commands::Keyring { username, action } => {
            (match action {
                KeyringCommand::SetToken => keyring::set_token,
                KeyringCommand::SetPassword => keyring::set_password,
                KeyringCommand::DeleteToken => keyring::delete_token,
                KeyringCommand::DeletePassword => keyring::delete_password,
            }(&vco_fqdn, &username))?
        }

        Commands::Property {
            creds_source,
            action,
        } => {
            let vco = client_from_creds(&vco_fqdn, &creds_source).await?;
            match action {
                PropertyCommand::List {
                    filter,
                    show_passwords,
                } => property::list(&vco, &filter, show_passwords).await?,
                _ => todo!(),
                // PropertyCommand::Get { name } => {}
                // PropertyCommand::Set => {}
                // PropertyCommand::Delete => {}
            }
        }

        Commands::GatewayMetric { creds_source } => {
            let vco = client_from_creds(&vco_fqdn, &creds_source).await?;

            let start = DateTime::from_rfc3339("2023-06-18T12:00:00Z").unwrap();
            let result = vco
                .get_gateway_status_metrics(
                    80,
                    &start,
                    None,
                    &[GatewayMetric::MemoryPct, GatewayMetric::CpuPct], //, GatewayMetric::ConnectedEdges],
                )
                .await;
            result?
        }
    };
    println!("{}", output_message);

    Ok(())
}
