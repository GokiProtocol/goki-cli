//! Utility functions.

use anyhow::{format_err, Result};
use colored::*;
use data_encoding::HEXLOWER;
use itertools::Itertools;
use sha2::{Digest, Sha256};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
    process::{Command, Output, Stdio},
    string::String,
};

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

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

fn fmt_command(command: &Command) -> String {
    rem_first_and_last(&format!("{:?}", command))
        .to_string()
        .split("\" \"")
        .map(|s| {
            if s.contains('?') || s.contains('!') {
                format!("'{}'", s.replace('\'', "\\'"))
            } else {
                s.to_string()
            }
        })
        .join(" ")
}

fn print_command(command: &Command) {
    println!(
        "{} {}",
        "=> Running command:".bold(),
        fmt_command(command).yellow()
    );
}

pub fn exec_command(command: &mut Command) -> Result<Output> {
    print_command(command);
    let exit = exec_command_unhandled(command)?;
    if !exit.status.success() {
        std::process::exit(exit.status.code().unwrap_or(1));
    }
    Ok(exit)
}

/// Executes a command, returning the captured stdout.
pub fn exec_command_with_output(command: &mut Command) -> Result<String> {
    print_command(command);
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

pub fn pause(message: &str) {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "{}", message).unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
