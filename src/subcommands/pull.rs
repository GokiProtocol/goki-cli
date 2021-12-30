use crate::location::fetch_program_file;
use crate::utils::sha256_digest;
use anyhow::Result;
use colored::*;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use tempfile::NamedTempFile;

pub async fn process(location: &str, out: Option<PathBuf>) -> Result<()> {
    let mut temp_out_file = NamedTempFile::new()?;
    let program_file_path = match out.clone() {
        Some(out_path) => {
            let mut out_file = File::create(&out_path)?;
            fetch_program_file(&mut out_file, location).await?;
            out_path
        }
        None => {
            fetch_program_file(&mut temp_out_file, location).await?;
            temp_out_file.path().to_path_buf()
        }
    };

    let input = File::open(&program_file_path)?;
    let mut reader = BufReader::new(input);
    let (program_file_size, program_file_digest) = sha256_digest(&mut reader)?;
    match out.clone() {
        Some(path) => {
            println!("Program buffer downloaded to {}.", path.display());
        }
        None => {
            println!("Program buffer downloaded.");
        }
    }
    println!("Size (bytes): {}", program_file_size.to_string().green());
    println!("SHA256: {}", program_file_digest.green());

    Ok(())
}
