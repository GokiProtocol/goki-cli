use anchor_client::Cluster;
use anyhow::{format_err, Result};
use colored::*;
use solana_sdk::pubkey::Pubkey;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use tempfile::NamedTempFile;

use crate::location::fetch_program_file;
use crate::solana_cmd;
use crate::utils::{gen_new_keypair, get_deployer_kp_path, sha256_digest};

pub async fn process(
    cluster: Cluster,
    upgrade_authority_kp_provided: Option<String>,
    location_or_buffer: String,
    program_id: String,
) -> Result<()> {
    let upgrade_authority_kp: String = match upgrade_authority_kp_provided {
        Some(kp_path) => kp_path,
        None => {
            if cluster == Cluster::Mainnet {
                return Err(format_err!(
                    "Must specify the --upgrade authority keypair on mainnet."
                ));
            }
            let deployer_kp = get_deployer_kp_path(&cluster)?;
            deployer_kp.display().to_string()
        }
    };

    let buffer_key: Pubkey = match Pubkey::from_str(location_or_buffer.as_str()) {
        Ok(buffer) => buffer,
        Err(_) => {
            let deployer_kp_path = get_deployer_kp_path(&cluster)?;

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

            solana_cmd::write_buffer(
                &cluster,
                &deployer_kp_path,
                program_file.path(),
                buffer_kp_file.path(),
            )?;

            buffer_key
        }
    };

    solana_cmd::upgrade(&cluster, &upgrade_authority_kp, &buffer_key, &program_id)?;

    Ok(())
}
