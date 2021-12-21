use crate::location::fetch_program_file;
use crate::utils::sha256_digest;
use anyhow::{format_err, Result};
use colored::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub async fn process(location: String, out: &Path) -> Result<()> {
    if out.exists() {
        return Err(format_err!("{} already exists", out.display()));
    }
    let mut program_file = File::create(out)?;
    fetch_program_file(&mut program_file, location.as_str()).await?;

    let input = File::open(out)?;
    let mut reader = BufReader::new(input);
    let (program_file_size, program_file_digest) = sha256_digest(&mut reader)?;
    println!(
        "Program buffer downloaded to {}.",
        out.display().to_string().yellow()
    );
    println!("Size (bytes): {}", program_file_size.to_string().green());
    println!("SHA256: {}", program_file_digest.green());

    Ok(())
}
