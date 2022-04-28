use anchor_client::Cluster;
use anyhow::{format_err, Result};
use std::{thread, time::Duration};

use crate::workspace::Workspace;

pub fn process(
    workspace: &Workspace,
    cluster: &Cluster,
    amount: &str,
    iterations: u32,
    interval: u64,
) -> Result<()> {
    if *cluster == Cluster::Mainnet {
        return Err(format_err!("cannot request an airdrop from mainnet"));
    }

    let ctx = workspace.new_cluster_context(cluster)?;
    let deployer = ctx.parse_wallet_alias("deployer")?;

    for i in 0..iterations {
        match ctx.exec_args(&["airdrop", amount], &deployer) {
            Ok(_) => {}
            Err(err) => {
                println!("Error performing airdrop: {}", err);
            }
        }
        if i != iterations - 1 {
            thread::sleep(Duration::from_millis(interval));
        }
    }

    Ok(())
}
