//! Goki entrypoint

use anchor_client::Cluster;
use anyhow::Result;
use clap::Parser;

#[derive(Debug, clap::Subcommand)]
pub enum SubCommand {
    #[clap(about = "Initializes a new Goki workspace.")]
    Init,
    #[clap(about = "Shows information about the Goki workspace.")]
    Show,
    #[clap(about = "Requests an airdrop of SOL from the Solana network.")]
    Airdrop {
        #[clap(short, long)]
        #[clap(help = "Cluster to request from.")]
        #[clap(default_value = "devnet")]
        cluster: Cluster,
        #[clap(help = "Airdrop request amount in SOL.")]
        #[clap(default_value = "1")]
        amount: String,
    },
    #[clap(about = "Uploads a Solana program buffer.")]
    UploadProgramBuffer {
        #[clap(short, long)]
        #[clap(help = "Cluster to deploy to.")]
        #[clap(default_value = "devnet")]
        cluster: Cluster,
        #[clap(short, long)]
        #[clap(help = "The path to the Solana program buffer.")]
        location: String,
        #[clap(short, long)]
        #[clap(
            help = "The program being upgraded. The buffer authority will be the program's current upgrade authority."
        )]
        program_id: String,
    },
    #[clap(about = "Deploys or upgrades a program using a local signer.")]
    DeployLocal {
        #[clap(short, long)]
        #[clap(help = "Cluster to deploy to.")]
        #[clap(default_value = "devnet")]
        cluster: Cluster,
        #[clap(short, long)]
        #[clap(
            help = "The keypair of the upgrade authority. If not provided, the deployer keypair will be used if not on mainnet."
        )]
        keypair: Option<String>,
        #[clap(short, long)]
        #[clap(help = "The public key of the program buffer.")]
        buffer: String,
        #[clap(short, long)]
        #[clap(
            help = "The program being upgraded. If deploying for the first time, you may specify a keypair."
        )]
        program_id: String,
    },
}

#[derive(Debug, clap::Parser)]
#[clap(about, version, author)]
pub struct Opts {
    #[clap(subcommand)]
    command: SubCommand,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    match opts.command {
        SubCommand::Init => {
            goki::subcommands::init::process()?;
        }
        SubCommand::Show => {
            goki::subcommands::show::process()?;
        }
        SubCommand::Airdrop { cluster, amount } => {
            goki::subcommands::airdrop::process(cluster, amount.as_str())?;
        }
        SubCommand::UploadProgramBuffer {
            cluster,
            location,
            program_id,
        } => {
            goki::subcommands::upload_program_buffer::process(cluster, location, program_id)
                .await?;
        }
        SubCommand::DeployLocal {
            cluster,
            keypair,
            buffer,
            program_id,
        } => {
            goki::subcommands::deploy_local::process(cluster, keypair, buffer, program_id)?;
        }
    }

    Ok(())
}
