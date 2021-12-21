use anchor_client::Cluster;
use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use std::{path::Path, process::Output};

use crate::utils::exec_command;

/// Writes a program buffer.
pub fn write_buffer(
    cluster: &Cluster,
    deployer_kp: &Path,
    program_file: &Path,
    buffer_kp_file: &Path,
) -> Result<Output> {
    exec_command(
        std::process::Command::new("solana")
            .arg("--url")
            .arg(&cluster.url())
            .arg("--keypair")
            .arg(deployer_kp)
            .arg("program")
            .arg("write-buffer")
            .arg(program_file)
            .arg("--buffer")
            .arg(buffer_kp_file),
    )
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
            .arg(&cluster.url())
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
