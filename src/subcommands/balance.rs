use anchor_client::Cluster;
use anyhow::Result;

use crate::workspace::Workspace;

pub fn process(workspace: &Workspace, cluster: &Cluster) -> Result<()> {
    let ctx = workspace.new_cluster_context(cluster)?;
    let deployer = ctx.parse_wallet_alias("deployer")?;

    println!("Deployer:");
    ctx.exec_args(&["account", &deployer], &deployer)?;

    Ok(())
}
