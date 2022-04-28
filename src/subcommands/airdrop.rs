use anchor_client::Cluster;
use anyhow::{format_err, Result};
use std::{thread, time::Duration};

use crate::workspace::Workspace;

pub fn process(
    workspace: &Workspace,
    cluster: Cluster,
    amount: &str,
    iterations: u32,
    interval: u64,
) -> Result<()> {
    if cluster == Cluster::Mainnet {
        return Err(format_err!("cannot request an airdrop from mainnet"));
    }

    for _ in 0..iterations {
        workspace.exec_deployer_command(&cluster, |cmd| {
            cmd.arg("airdrop").arg(amount);
            Ok(())
        })?;

        thread::sleep(Duration::from_millis(interval));
    }

    Ok(())
}
