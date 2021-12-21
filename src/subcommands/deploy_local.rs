use anchor_client::Cluster;
use anyhow::{format_err, Result};
use colored::*;
use solana_sdk::pubkey::Pubkey;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tempfile::NamedTempFile;

use crate::solana_cmd;
use crate::utils::{gen_new_keypair, get_deployer_kp_path, sha256_digest};
use crate::{location::fetch_program_file, utils::exec_command};

pub async fn process(
    cluster: Cluster,
    keypair_provided: Option<String>,
    location_or_buffer: String,
    program_id: String,
) -> Result<()> {
    let upgrade_authority_kp: String = match keypair_provided {
        Some(kp_path) => kp_path,
        None => {
            if cluster == Cluster::Mainnet {
                return Err(format_err!(
                    "Must specify the upgrade authority keypair on mainnet."
                ));
            }
            if !PathBuf::from(".goki/deployers/").exists() {
                return Err(format_err!(".goki/deployers/ does not exist"));
            }
            let path_string = format!(".goki/deployers/{}.json", cluster);
            let deployer_kp = Path::new(path_string.as_str());
            if !deployer_kp.exists() {
                return Err(format_err!(
                    "{} keypair not found at path {}",
                    cluster,
                    deployer_kp.display()
                ));
            }
            deployer_kp.display().to_string()
        }
    };

    let buffer_key: Pubkey = match Pubkey::from_str(location_or_buffer.as_str()) {
        Ok(buffer) => buffer,
        Err(_) => {
            let mut program_file = NamedTempFile::new()?;
            fetch_program_file(&mut program_file, location_or_buffer.as_str()).await?;

            let input = File::open(program_file.path())?;
            let mut reader = BufReader::new(input);
            let (program_file_size, program_file_digest) = sha256_digest(&mut reader)?;
            println!("Program buffer downloaded.");
            println!("Size (bytes): {}", program_file_size.to_string().green());
            println!("SHA256: {}", program_file_digest.green());

            let mut buffer_kp_file = NamedTempFile::new()?;
            let buffer_key = gen_new_keypair(&mut buffer_kp_file)?;

            let deployer_kp_path = get_deployer_kp_path(&cluster)?;
            solana_cmd::write_buffer(
                &cluster,
                &deployer_kp_path,
                program_file.path(),
                buffer_kp_file.path(),
            )?;

            buffer_key
        }
    };

    exec_command(
        std::process::Command::new("solana")
            .arg("--url")
            .arg(cluster.url())
            .arg("--keypair")
            .arg(upgrade_authority_kp.clone())
            .arg("program")
            .arg("deploy")
            .arg("--buffer")
            .arg(buffer_key.to_string())
            .arg("--program-id")
            .arg(program_id)
            .arg("--upgrade_authority")
            .arg(upgrade_authority_kp),
    )?;

    Ok(())
}
