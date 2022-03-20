//! Calls to the `solana` CLI.
use anchor_client::Cluster;
use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use std::{
    path::Path,
    process::{Command, Output},
};

use crate::{
    utils::{exec_command, get_cluster_url},
    workspace::Workspace,
};

/// Sets the buffer authority of a buffer.
pub fn set_buffer_authority(
    workspace: &Workspace,
    cluster: &Cluster,
    buffer_key: &Pubkey,
    authority: &str,
) -> Result<Output> {
    workspace.exec_deployer_command(cluster, |cmd: &mut Command| {
        cmd.arg("program")
            .arg("set-buffer-authority")
            .arg(buffer_key.to_string())
            .arg("--new-buffer-authority")
            .arg(authority);
        Ok(())
    })
}

/// Sets the upgrade authority of a program.
pub fn set_upgrade_authority(
    cluster: &Cluster,
    program_id: &Pubkey,
    current_authority: &Path,
    new_authority: &str,
) -> Result<Output> {
    exec_command(
        std::process::Command::new("solana")
            .arg("--url")
            .arg(get_cluster_url(cluster)?)
            .arg("--keypair")
            .arg(current_authority)
            .arg("program")
            .arg("set-upgrade-authority")
            .arg(program_id.to_string())
            .arg("--new-upgrade-authority")
            .arg(new_authority),
    )
}

/// Writes a program buffer.
pub fn write_buffer(
    workspace: &Workspace,
    cluster: &Cluster,
    program_file: &Path,
    buffer_kp_file: &Path,
) -> Result<Output> {
    workspace.exec_deployer_command(cluster, |cmd| {
        cmd.arg("program")
            .arg("write-buffer")
            .arg(program_file)
            .arg("--buffer")
            .arg(buffer_kp_file);
        Ok(())
    })
}

/// Deploys a program.
pub fn deploy(
    workspace: &Workspace,
    cluster: &Cluster,
    program_file: &Path,
    program_kp_path: &Path,
) -> Result<Output> {
    workspace.exec_deployer_command(cluster, |cmd| {
        cmd.arg("program")
            .arg("deploy")
            .arg("--program-id")
            .arg(program_kp_path)
            .arg(program_file);
        Ok(())
    })
}

/// Upgrades a program.
pub fn upgrade(
    cluster: &Cluster,
    upgrade_authority_kp: &str,
    buffer_key: &Pubkey,
    program_id: &str,
) -> Result<Output> {
    exec_command(
        std::process::Command::new("solana")
            .arg("--url")
            .arg(get_cluster_url(cluster)?)
            .arg("--keypair")
            .arg(upgrade_authority_kp)
            .arg("program")
            .arg("deploy")
            .arg("--buffer")
            .arg(buffer_key.to_string())
            .arg("--program-id")
            .arg(program_id)
            .arg("--upgrade-authority")
            .arg(upgrade_authority_kp),
    )
}

pub fn new_solana_cmd() -> Command {
    Command::new("solana")
}
