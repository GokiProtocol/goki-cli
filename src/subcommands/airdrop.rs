use anchor_client::Cluster;
use anyhow::{format_err, Result};

use crate::workspace::Workspace;

pub fn process(workspace: &Workspace, cluster: Cluster, amount: &str) -> Result<()> {
    if cluster == Cluster::Mainnet {
        return Err(format_err!("cannot request an airdrop from mainnet"));
    }

    workspace.exec_deployer_command(&cluster, |cmd| {
        cmd.arg("airdrop").arg(amount);
        Ok(())
    })?;

    Ok(())
}
