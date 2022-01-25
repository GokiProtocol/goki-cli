use anchor_client::Cluster;
use anyhow::{format_err, Result};
use std::path::{Path, PathBuf};

use crate::utils::{exec_command, get_cluster_url};

pub fn process(cluster: Cluster, amount: &str) -> Result<()> {
    if cluster == Cluster::Mainnet {
        return Err(format_err!("cannot request an airdrop from mainnet"));
    }

    if !PathBuf::from(".goki/deployers/").exists() {
        return Err(format_err!(".goki/deployers/ does not exist"));
    }

    let path_string = format!(".goki/deployers/{}.json", cluster);
    let keypair_path = Path::new(path_string.as_str());
    if !keypair_path.exists() {
        return Err(format_err!(
            "{} keypair not found at path {}",
            cluster,
            keypair_path.display()
        ));
    }

    exec_command(
        std::process::Command::new("solana")
            .arg("--url")
            .arg(get_cluster_url(&cluster)?)
            .arg("--keypair")
            .arg(keypair_path)
            .arg("airdrop")
            .arg(amount),
    )?;

    Ok(())
}
