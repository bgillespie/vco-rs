use anyhow::Result;
use clap::{Args, Parser, Subcommand};

// TODO this api_v1 stuff should be in `client` at best and not here.
use api_v1::date_time::DateTime;
use api_v1::gateway::GatewayMetric;
use client::client::Client as VcoClient;

mod keyring;
mod property;

/// Build a `VcoClient` given the VCO's FQDN and credentials.
async fn client_from_creds(vco_fqdn: &str, creds_source: &CredentialSource) -> Result<VcoClient> {
    let vco = if creds_source.is_token() {
        let (_, token) = creds_source.acquire(&vco_fqdn)?;
        VcoClient::operator_login_token(&vco_fqdn, &token)
            .await
            .map_err(|_| {
                anyhow::format_err!("Could not log into {vco_fqdn} with the given token.")
            })?
    }
    else if creds_source.is_password() {
        let (username, password) = creds_source.acquire(&vco_fqdn)?;
        VcoClient::operator_login_password(&vco_fqdn, &username, &password)
            .await
            .map_err(|e| {
                anyhow::format_err!(
                    "Could not log into {vco_fqdn} as {username} with the given password...\n{e:?}."
                )
            })?
    }
    else {
        unreachable!()
    };
    Ok(vco)
}

/// Check to see if the passed-in string is an email or not.
fn is_email(value: &str) -> Result<String, String> {
    let parts = value.split("@").collect::<Vec<&str>>();
    if parts.len() != 2 || !parts[1].contains(".") {
        Err(format!("Value is not a valid email: {value}"))
    } else {
        Ok(value.to_string())
    }
}

/// Arguments for the source of VCO credentials.
///
/// The `CredentialSource` _implementation_ has methods to obtain and return those credentials, for
/// example by prompting on the CLI.
///
/// This struct defines multiple mutually-exclusive options for sources of credentials and is
/// included in several places in the Command-Line Interface definitions below.
#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct CredentialSource {
    /// Prompt for the password of this user on the command-line.
    #[arg(long, value_name = "USERNAME")]
    prompt: Option<String>,

    /// Prompt for an API token on the command-line.
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
            // Prompt on the command line for the user's password.
            Ok((
                username.to_string(),
                rpassword::prompt_password(&format!("Password for {username} on {vco_fqdn}: "))?,
            ))
        } else if self.token {
            // Prompt on the command line for a token on the VCO.
            // We don't need the user name here; the VCO knows who owns it.
            Ok((
                String::new(),
                rpassword::prompt_password(&format!("API token for {vco_fqdn}: "))?,
            ))
        } else if let Some(username) = &self.keyring_token {
            // Get the user's token from the system keyring, if it exists.
            Ok((
                username.to_string(),
                keyring::get_token(&vco_fqdn, &username)?,
            ))
        } else if let Some(username) = &self.keyring_password {
            // Get the user's password from the system token, if it exists.
            Ok((
                username.to_string(),
                keyring::get_password(&vco_fqdn, &username)?,
            ))
        } else {
            // There may be other sources in future...
            unreachable!()
        }
    }

    // Is the credential a token?
    fn is_token(&self) -> bool {
        self.token || self.keyring_token.is_some()
    }

    // Is the credential a password?
    fn is_password(&self) -> bool {
        self.prompt.is_some() || self.keyring_password.is_some()
    }
}

//
// Defining the Command-Line Interface.
//

/// Base of the CLI command tree.
#[derive(Debug, Parser)]
#[command(name = "vcoctl")]
#[command(about = "CLI tool for interacting with VMware SD-WAN Orchestrator")]
struct Cli {
    vco_fqdn: String,

    #[command(subcommand)]
    command: Commands,
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

    /// Actions on VCG metrics.
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
    /// List system properties.
    List {
        /// If specified, this string will be used to filter the _names_ of properties.
        #[arg(long, required = false, default_value = "")]
        filter: String,

        /// If `false`, this will prevent any properties with the `isPassword` setting to be
        /// redacted in the output.
        #[arg(long, required = false, default_value = "false")]
        show_passwords: bool,
    },

    /// Get a specific system property.
    Get {
        name: String,
    },

    /// Set a system property.
    Set,

    /// Delete a system property.
    Delete,
}


/// This is the entry point to this CLI program.
/// TODO return an appropriate value to the terminal emulator on error, or `0` in success.
#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let vco_fqdn = args.vco_fqdn;

    // Take action depending on the parameters passed in, and wait for some sort of output to print.
    // The actual action code is held in separate modules in this `cli` crate.
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
