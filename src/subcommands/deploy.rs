use anchor_client::Cluster;
use anyhow::{format_err, Result};
use colored::*;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;
use tempfile::NamedTempFile;

use crate::location::fetch_program_file;
use crate::solana_cmd;
use crate::utils::{get_deployer_kp_path, sha256_digest};

pub async fn process(
    cluster: Cluster,
    upgrade_authority_provided: Option<String>,
    location_or_buffer: String,
    program_kp_path: &Path,
) -> Result<()> {
    let deployer_kp_path = get_deployer_kp_path(&cluster)?;
    let program_kp = solana_sdk::signature::read_keypair_file(program_kp_path)
        .map_err(|e| format_err!("could not open program kp path: {}", e))?;

    let upgrade_authority: Pubkey = match upgrade_authority_provided {
        Some(pubkey_str) => Pubkey::from_str(&pubkey_str)?,
        None => {
            if cluster == Cluster::Mainnet {
                return Err(format_err!(
                    "Must specify the --upgrade authority public key on mainnet."
                ));
            }
            let deployer_kp = solana_sdk::signature::read_keypair_file(&deployer_kp_path)
                .map_err(|_| format_err!("could not open deployer KP"))?;
            deployer_kp.try_pubkey()?
        }
    };

    let mut program_file = NamedTempFile::new()?;
    fetch_program_file(&mut program_file, location_or_buffer.as_str()).await?;

    let input = File::open(program_file.path())?;
    let mut reader = BufReader::new(input);
    let (program_file_size, program_file_digest) = sha256_digest(&mut reader)?;
    println!("Program buffer downloaded.");
    println!("Size (bytes): {}", program_file_size.to_string().green());
    println!("SHA256: {}", program_file_digest.green());

    solana_cmd::deploy(&cluster, program_file.path(), program_kp_path)?;
    solana_cmd::set_upgrade_authority(
        &cluster,
        &program_kp.pubkey(),
        &deployer_kp_path,
        &upgrade_authority.to_string(),
    )?;

    Ok(())
}
