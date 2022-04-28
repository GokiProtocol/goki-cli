use anchor_client::Cluster;
use anyhow::{format_err, Result};
use colored::*;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file, signer::Signer};
use std::fs::File;
use std::io::Write;
use std::{fs, path::Path};

use crate::utils::{exec_command, gen_keypair_file};
use crate::{config::Config, workspace::Workspace};

pub fn process(path: &Path) -> Result<()> {
    let maybe_cfg = Config::discover()?;
    let cfg = if let Some(cfg) = maybe_cfg {
        println!("Goki.toml already exists in workspace");
        cfg.into_inner()
    } else {
        let cfg = Config::default();
        let mut file = File::create("Goki.toml")?;
        file.write_all(cfg.to_string().as_bytes())?;
        cfg
    };

    let workspace = &Workspace {
        path: path.to_path_buf(),
        cfg,
    };

    fs::create_dir_all(workspace.deployer_dir())?;

    let mut result: Vec<(Cluster, Pubkey)> = vec![];

    for cluster in [Cluster::Devnet, Cluster::Testnet, Cluster::Mainnet].iter() {
        let keypair_path = workspace.get_deployer_kp_path(cluster);
        let key = if keypair_path.exists() {
            let kp = read_keypair_file(keypair_path)
                .map_err(|_| format_err!("could not read keypair"))?;
            let pubkey = kp.pubkey();
            println!("Keypair at {} already exists: {}", cluster, pubkey);
            pubkey
        } else {
            gen_keypair_file(&keypair_path)?
        };
        result.push((cluster.clone(), key));
    }

    println!("{}", "Deployers:".bold());
    for (cluster, key) in result.iter() {
        println!("{}: {}", cluster, key);
    }

    let workspace = workspace.reload()?;

    for (cluster, _key) in result.iter() {
        if cluster.clone() != Cluster::Mainnet {
            let path_string = format!(".goki/deployers/{}.json", cluster);
            let keypair_path = Path::new(path_string.as_str());
            exec_command(
                std::process::Command::new("solana")
                    .arg("--url")
                    .arg(workspace.get_cluster_url(cluster)?)
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
