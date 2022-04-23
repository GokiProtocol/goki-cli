use crate::workspace::Workspace;
use anchor_client::Cluster;
use anyhow::Result;

pub fn process(
    workspace: &Workspace,
    cluster: &Cluster,
    from_raw: &str,
    to_raw: &str,
    amount: &str,
) -> Result<()> {
    let ctx = workspace.new_cluster_context(cluster)?;
    let to = ctx.parse_wallet_alias(to_raw)?;
    ctx.exec_args(
        &["transfer", &to, amount],
        &ctx.parse_wallet_alias(from_raw)?,
    )?;
    Ok(())
}
