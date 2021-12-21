use anyhow::format_err;
use anyhow::Result;
use std::fs::File;
use std::io::copy;
use std::io::Write;
use std::path::PathBuf;

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

pub async fn fetch_program_file<W: Write>(program_file: &mut W, location: &str) -> Result<()> {
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
