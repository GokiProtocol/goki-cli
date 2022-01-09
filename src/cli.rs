//! Goki entrypoint

use crate::subcommands;
use anchor_client::Cluster;
use anyhow::Result;
use clap::ArgSettings::NextLineHelp;
use clap::Parser;
use std::path::PathBuf;

const LOCATION_HELP: &str =
    "The location of the Solana program binary. This can be in one of the following formats:

- path, for example `./path/to/program.so`
- URL, for example `https://github.com/GokiProtocol/goki/releases/download/v0.5.2/smart_wallet.so`
- GitHub artifact, for example `gh:smart_wallet:GokiProtocol/goki@0.5.2`
- Solana Program Registry artifact, for example `spr:QuarryProtocol/quarry_mine`
";

#[derive(Debug, clap::Subcommand)]
pub enum SubCommand {
    /// Initializes a new Goki workspace.
    Init,
    /// Shows information about the Goki workspace.
    Show,
    /// Requests an airdrop of SOL from the Solana network.
    Airdrop {
        /// Cluster to request from.
        #[clap(short, long)]
        #[clap(possible_value("devnet"), possible_value("testnet"))]
        #[clap(default_value = "devnet")]
        cluster: Cluster,

        /// Airdrop request amount in SOL.
        #[clap(default_value = "1")]
        amount: String,
    },
    /// Uploads a Solana program buffer.
    UploadProgramBuffer {
        /// Cluster to deploy to.
        #[clap(short, long)]
        #[clap(default_value = "devnet")]
        cluster: Cluster,

        #[clap(short, long)]
        #[clap(help = LOCATION_HELP)]
        #[clap(setting = NextLineHelp)]
        location: String,

        /// The program being upgraded.
        ///
        /// The buffer authority will be set to the program's current upgrade authority.
        #[clap(short, long)]
        program_id: String,
    },

    /// Deploys a program for the first time.
    Deploy {
        /// Cluster to deploy to.
        #[clap(short, long)]
        #[clap(default_value = "devnet")]
        cluster: Cluster,

        /// The public key of the upgrade authority. If not provided, the deployer key will be used if not on mainnet.
        #[clap(short, long)]
        upgrade_authority: Option<String>,

        #[clap(short, long)]
        #[clap(help = LOCATION_HELP)]
        #[clap(setting = NextLineHelp)]
        location: String,

        /// The path to the keypair of the program being deployed.
        #[clap(short, long)]
        program_kp: PathBuf,
    },
    /// Upgrades a program using a local signer.
    UpgradeLocal {
        /// Cluster to deploy to.
        #[clap(short, long)]
        #[clap(default_value = "devnet")]
        cluster: Cluster,

        /// The keypair of the upgrade authority.
        ///
        /// If not provided, the deployer keypair will be used if not on mainnet.
        #[clap(short, long)]
        upgrade_authority_keypair: Option<String>,

        /// The path to the Solana program bytecode. If a public key is provided, this will use an already uploaded program buffer.
        #[clap(short, long)]
        location: String,

        /// The program being upgraded.
        #[clap(short, long)]
        program_id: String,
    },
    /// Pulls a binary from a location.
    Pull {
        #[clap(help = LOCATION_HELP)]
        #[clap(setting = NextLineHelp)]
        location: String,

        /// Output path of the program binary.
        ///
        /// If not specified, the program binary will not be written.
        #[clap(short, long)]
        out: Option<PathBuf>,
    },
}

#[derive(Debug, clap::Parser)]
#[clap(about, version, author)]
pub struct Opts {
    #[clap(subcommand)]
    pub command: SubCommand,
}

impl SubCommand {
    /// Runs the subcommand.
    pub async fn run(self) -> Result<()> {
        match self {
            SubCommand::Init => {
                subcommands::init::process()?;
            }
            SubCommand::Show => {
                subcommands::show::process()?;
            }
            SubCommand::Airdrop { cluster, amount } => {
                subcommands::airdrop::process(cluster, amount.as_str())?;
            }
            SubCommand::UploadProgramBuffer {
                cluster,
                location,
                program_id,
            } => {
                subcommands::upload_program_buffer::process(cluster, location, program_id).await?;
            }
            SubCommand::Deploy {
                cluster,
                upgrade_authority,
                location,
                program_kp,
            } => {
                subcommands::deploy::process(cluster, upgrade_authority, location, &program_kp)
                    .await?;
            }
            SubCommand::UpgradeLocal {
                cluster,
                upgrade_authority_keypair,
                location,
                program_id,
            } => {
                subcommands::upgrade_local::process(
                    cluster,
                    upgrade_authority_keypair,
                    location,
                    program_id,
                )
                .await?;
            }
            SubCommand::Pull { location, out } => {
                subcommands::pull::process(&location, out).await?;
            }
        };

        Ok(())
    }
}

/// Runs the CLI.
pub async fn run() -> Result<()> {
    let opts: Opts = Opts::parse();
    opts.command.run().await?;
    Ok(())
}
