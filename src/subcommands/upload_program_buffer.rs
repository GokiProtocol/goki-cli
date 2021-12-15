use crate::utils::exec_command;
use crate::utils::exec_command_with_output;
use crate::utils::gen_new_keypair;
use crate::utils::print_header;
use anchor_client::Cluster;
use anyhow::format_err;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::copy;
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

fn parse_gh_ref(location: &str) -> Option<String> {
    let (_, raw_gh_ref) = location.split_once("gh:")?;
    let (program, rest) = raw_gh_ref.split_once(':')?;
    let (repo, version) = rest.split_once('@')?;
    let target = format!(
        "https://github.com/{}/releases/download/v{}/{}.so",
        repo, version, program
    );
    Some(target)
}

async fn download_file<W: Write>(program_file: &mut W, target: String) -> Result<()> {
    println!("Downloading program code from {}", target);
    let response = reqwest::get(target).await?;
    if !response.status().is_success() {
        return Err(format_err!("program file not found"));
    }
    let bytes: Vec<u8> = response.bytes().await?.into_iter().collect();
    program_file.write_all(&bytes)?;
    Ok(())
}

async fn fetch_program_file<W: Write>(program_file: &mut W, location: &str) -> Result<()> {
    if location.starts_with("gh:") {
        let target = parse_gh_ref(location).ok_or_else(|| {
            format_err!(
                "invalid gh format: should look like `gh:smart_wallet:GokiProtocol/goki@0.5.2`"
            )
        })?;
        return download_file(program_file, target).await;
    }
    if location.starts_with("https://") || location.starts_with("http://") {
        return download_file(program_file, location.into()).await;
    }

    let mut file = File::open(&PathBuf::from(location))?;
    copy(&mut file, program_file)?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct ProgramInfo {
    pub authority: String,
}

pub async fn process(cluster: Cluster, location: String, program_id: String) -> Result<()> {
    // let mut program_file = NamedTempFile::new()?;
    let mut program_file = File::create(PathBuf::from("test.so"))?;
    fetch_program_file(&mut program_file, location.as_str()).await?;

    let mut buffer_kp_file = NamedTempFile::new()?;
    let buffer_key = gen_new_keypair(&mut buffer_kp_file)?;

    let program_info_output = exec_command_with_output(
        std::process::Command::new("solana")
            .arg("--url")
            .arg(&cluster.url())
            .arg("--keypair")
            .arg(format!(".goki/deployers/{}.json", cluster))
            .arg("program")
            .arg("show")
            .arg(&program_id)
            .arg("--output")
            .arg("json-compact"),
    )?;
    let program_info: ProgramInfo = serde_json::from_str(program_info_output.as_str())?;

    println!("Program ID: {}", program_id);
    println!("Program authority: {}", program_info.authority);
    println!("Buffer key: {}", buffer_key);

    print_header("Writing buffer");

    exec_command(
        std::process::Command::new("solana")
            .arg("--url")
            .arg(&cluster.url())
            .arg("--keypair")
            .arg(format!(".goki/deployers/{}.json", cluster))
            .arg("program")
            .arg("write-buffer")
            // .arg(program_file.path())
            .arg("test.so")
            .arg("--buffer")
            .arg(buffer_kp_file.path()),
    )?;

    print_header("Setting buffer authority");

    exec_command(
        std::process::Command::new("solana")
            .arg("--url")
            .arg(&cluster.url())
            .arg("--keypair")
            .arg(format!(".goki/deployers/{}.json", cluster))
            .arg("program")
            .arg("set-buffer-authority")
            .arg(buffer_key.to_string())
            .arg("--new-buffer-authority")
            .arg(&program_info.authority),
    )?;

    Ok(())
}
