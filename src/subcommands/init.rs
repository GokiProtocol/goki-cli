use crate::config::*;
use crate::utils::{exec_command, gen_keypair_file};
use anchor_client::Cluster;
use anyhow::{anyhow, format_err, Result};
use colored::*;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file, signer::Signer};
use std::fs::File;
use std::io::Write;
use std::{fs, path::Path};

pub fn process() -> Result<()> {
    if Config::discover()?.is_some() {
        return Err(anyhow!("Goki already initialized."));
    }

    fs::create_dir_all(".goki/deployers/")?;

    let toml = Config::default();
    let mut file = File::create("Goki.toml")?;
    file.write_all(toml.as_bytes())?;

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
    }

    println!("{}", "Deployers:".bold());
    for (cluster, key) in result.iter() {
        println!("{}: {}", cluster, key);
    }

    for (cluster, _key) in result.iter() {
        if cluster.clone() != Cluster::Mainnet {
            let path_string = format!(".goki/deployers/{}.json", cluster);
            let keypair_path = Path::new(path_string.as_str());
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

    println!("Goki initialized! Please add the .goki/ directory to your gitignore.");

    Ok(())
}
