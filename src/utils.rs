use anchor_client::Cluster;
use anyhow::{format_err, Result};
use colored::*;
use data_encoding::HEXLOWER;
use sha2::{Digest, Sha256};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;

/// Generates a keypair and writes it to the [Write].
pub fn gen_new_keypair<W: Write>(write: &mut W) -> Result<Pubkey> {
    let new_keypair = solana_sdk::signer::keypair::Keypair::new();
    let new_key = new_keypair.pubkey();
    solana_sdk::signer::keypair::write_keypair(&new_keypair, write)
        .map_err(|_| format_err!("could not generate keypair"))?;
    Ok(new_key)
}

/// Generates a keypair at a [Path].
pub fn gen_keypair_file(path: &Path) -> Result<Pubkey> {
    let mut file = File::create(path)?;
    let pubkey = gen_new_keypair(&mut file)?;
    Ok(pubkey)
}

pub fn print_header(header: &'static str) {
    println!();
    println!("{}", "===================================".bold());
    println!();
    println!("    {}", header.bold());
    println!();
    println!("{}", "===================================".bold());
    println!();
}

pub fn exec_command_unhandled(command: &mut Command) -> Result<Output> {
    command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| format_err!("Error deploying: {}", e.to_string()))
}

pub fn exec_command(command: &mut Command) -> Result<Output> {
    println!("Running command: {:?}", command);
    let exit = exec_command_unhandled(command)?;
    if !exit.status.success() {
        std::process::exit(exit.status.code().unwrap_or(1));
    }
    Ok(exit)
}

/// Executes a command, returning the captured stdout.
pub fn exec_command_with_output(command: &mut Command) -> Result<String> {
    let exit = command
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| format_err!("Error deploying: {}", e.to_string()))?;
    if !exit.status.success() {
        std::process::exit(exit.status.code().unwrap_or(1));
    }
    Ok(String::from_utf8(exit.stdout)?)
}

pub fn sha256_digest<R: Read>(reader: &mut R) -> Result<(u64, String)> {
    let mut hasher = Sha256::new();
    let num_bytes = io::copy(reader, &mut hasher)?;
    let hash_bytes = hasher.finalize();
    Ok((num_bytes, HEXLOWER.encode(hash_bytes.as_ref())))
}

pub fn get_deployer_kp_path(cluster: &Cluster) -> Result<PathBuf> {
    if !PathBuf::from(".goki/deployers/").exists() {
        return Err(format_err!(
            ".goki/deployers/ does not exist; you may need to run `goki init`"
        ));
    }
    let deployer_kp_string = format!(".goki/deployers/{}.json", cluster);
    let deployer_kp_path = &PathBuf::from(deployer_kp_string.as_str());
    if !deployer_kp_path.exists() {
        return Err(format_err!(
            "Deployer not found at {:?}; you may need to run `goki init`",
            deployer_kp_path
        ));
    }
    Ok(deployer_kp_path.clone())
}
