use anchor_client::Cluster;
use anyhow::{format_err, Result};
use colored::*;
use solana_sdk::pubkey::Pubkey;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use tempfile::NamedTempFile;

use crate::utils::{gen_new_keypair, sha256_digest};
use crate::{location::fetch_program_file, workspace::Workspace};

pub async fn process(
    workspace: &Workspace,
    cluster: Cluster,
    upgrade_authority_kp_provided: Option<String>,
    location_or_buffer: String,
    program_id: String,
) -> Result<()> {
    let upgrade_authority_kp: String = match &upgrade_authority_kp_provided {
        Some(kp_path) => kp_path.clone(),
        None => match &workspace.cfg.upgrade_authority_keypair {
            Some(kp_path) => kp_path.clone(),
            None => {
                if cluster == Cluster::Mainnet {
                    return Err(format_err!(
                        "Must specify the --upgrade_authority_keypair on mainnet."
                    ));
                }
                let deployer_kp = workspace.get_deployer_kp_path_if_exists(&cluster)?;
                deployer_kp.as_path().display().to_string()
            }
        },
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

            workspace.write_buffer(&cluster, program_file.path(), buffer_kp_file.path())?;
            workspace.set_buffer_authority(&cluster, &buffer_key, &upgrade_authority_kp)?;

            buffer_key
        }
    };

    workspace.upgrade(&cluster, &upgrade_authority_kp, &buffer_key, &program_id)?;

    Ok(())
}
