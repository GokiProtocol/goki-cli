use crate::workspace::Workspace;
use anchor_client::Cluster;
use anyhow::Result;

pub fn process(workspace: &Workspace, cluster: Cluster, to_raw: &str, amount: &str) -> Result<()> {
    let ctx = workspace.new_upgrader_context(&cluster);
    let to = match to_raw {
        "deployer" => ctx.get_deployer_kp_path().display().to_string(),
        _ => to_raw.to_string(),
    };
    ctx.exec_args(&["transfer", &to, amount])?;
    Ok(())
}
