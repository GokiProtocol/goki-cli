use anchor_client::Cluster;
use anyhow::Result;
use std::{path::Path, process::Output};

use crate::utils::exec_command;

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
