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
    #[clap(about = "Uploads a Solana program buffer.")]
    UploadProgramBuffer {
        #[clap(short, long)]
        #[clap(help = "Cluster to deploy to. Defaults to devnet.")]
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
        SubCommand::UploadProgramBuffer {
            cluster,
            location,
            program_id,
        } => {
            goki::subcommands::upload_program_buffer::process(cluster, location, program_id)
                .await?;
        }
    }

    Ok(())
}
