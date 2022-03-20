use anchor_client::Cluster;
use anyhow::{format_err, Result};
use colored::*;
use solana_sdk::{signature::read_keypair_file, signer::Signer};

use crate::workspace::Workspace;

pub fn process(workspace: &Workspace) -> Result<()> {
    let deployer_dir = workspace.deployer_dir();
    if !deployer_dir.exists() {
        return Err(format_err!("{} does not exist", deployer_dir.display()));
    }

    println!("{}", "Deployers:".bold());
    for cluster in [Cluster::Devnet, Cluster::Testnet, Cluster::Mainnet].iter() {
        let keypair_path = workspace.get_deployer_kp_path(cluster);
        if keypair_path.exists() {
            let kp = read_keypair_file(keypair_path)
                .map_err(|_| format_err!("could not read keypair"))?;
            let pubkey = kp.pubkey();
            println!("{}: {}", cluster, pubkey);
        } else {
            println!("{}: {}", cluster, "not found".red());
        };
    }

    Ok(())
}
