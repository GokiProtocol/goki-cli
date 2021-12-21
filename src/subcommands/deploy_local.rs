use anchor_client::Cluster;
use anyhow::{format_err, Result};
use std::path::{Path, PathBuf};

use crate::utils::exec_command;

pub fn process(
    cluster: Cluster,
    keypair_provided: Option<String>,
    buffer: String,
    program_id: String,
) -> Result<()> {
    if cluster == Cluster::Mainnet {
        return Err(format_err!("cannot request an airdrop from mainnet"));
    }

    let keypair: String = match keypair_provided {
        Some(kp_path) => kp_path,
        None => {
            if cluster == Cluster::Mainnet {
                return Err(format_err!(
                    "Must specify the upgrade authority keypair on mainnet."
                ));
            }
            if !PathBuf::from(".goki/deployers/").exists() {
                return Err(format_err!(".goki/deployers/ does not exist"));
            }
            let path_string = format!(".goki/deployers/{}.json", cluster);
            let deployer_kp = Path::new(path_string.as_str());
            if !deployer_kp.exists() {
                return Err(format_err!(
                    "{} keypair not found at path {}",
                    cluster,
                    deployer_kp.display()
                ));
            }
            deployer_kp.display().to_string()
        }
    };

    exec_command(
        std::process::Command::new("solana")
            .arg("--url")
            .arg(cluster.url())
            .arg("--keypair")
            .arg(keypair.clone())
            .arg("program")
            .arg("deploy")
            .arg("--buffer")
            .arg(buffer)
            .arg("--program-id")
            .arg(program_id)
            .arg("--upgrade_authority")
            .arg(keypair),
    )?;

    Ok(())
}
