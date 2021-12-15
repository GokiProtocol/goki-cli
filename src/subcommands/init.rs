use crate::utils::{exec_command, gen_keypair_file};
use anchor_client::Cluster;
use anyhow::{format_err, Result};
use colored::*;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file, signer::Signer};
use std::{fs, path::Path};

pub fn process() -> Result<()> {
    fs::create_dir_all(".goki/deployers/")?;

    let mut result: Vec<(Cluster, Pubkey)> = vec![];

    for cluster in [Cluster::Devnet, Cluster::Testnet, Cluster::Mainnet].iter() {
        let path_string = format!(".goki/deployers/{}.json", cluster);
        let keypair_path = Path::new(path_string.as_str());
        let key = if keypair_path.exists() {
            let kp = read_keypair_file(keypair_path)
                .map_err(|_| format_err!("could not read keypair"))?;
            let pubkey = kp.pubkey();
            println!("Keypair at {} already exists: {}", cluster, pubkey);
            pubkey
        } else {
            gen_keypair_file(keypair_path)?
        };
        result.push((cluster.clone(), key));

        if cluster.clone() != Cluster::Mainnet {
            exec_command(
                std::process::Command::new("solana")
                    .arg("--url")
                    .arg(cluster.url())
                    .arg("--keypair")
                    .arg(keypair_path)
                    .arg("airdrop")
                    .arg("1"),
            )?;
        }
    }

    println!("{}", "Deployers:".bold());
    for (cluster, key) in result.iter() {
        println!("{}: {}", cluster, key);
    }

    println!("Goki initialized! Please add the .goki/ directory to your gitignore.");

    Ok(())
}
