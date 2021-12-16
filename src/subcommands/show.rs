use anchor_client::Cluster;
use anyhow::{format_err, Result};
use colored::*;
use solana_sdk::{signature::read_keypair_file, signer::Signer};
use std::path::{Path, PathBuf};

pub fn process() -> Result<()> {
    if !PathBuf::from(".goki/deployers/").exists() {
        return Err(format_err!(".goki/deployers/ does not exist"));
    }

    println!("{}", "Deployers:".bold());
    for cluster in [Cluster::Devnet, Cluster::Testnet, Cluster::Mainnet].iter() {
        let path_string = format!(".goki/deployers/{}.json", cluster);
        let keypair_path = Path::new(path_string.as_str());
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
