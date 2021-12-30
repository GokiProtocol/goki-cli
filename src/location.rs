//! Parses and fetches program locations.
use anyhow::{format_err, Result};
use std::{
    fs::File,
    io::{copy, Write},
    path::PathBuf,
};

async fn download_file<W: Write>(program_file: &mut W, target: &str) -> Result<()> {
    println!("Downloading program code from {}", target);
    let response = reqwest::get(target).await?;
    if !response.status().is_success() {
        return Err(format_err!("program file not found"));
    }
    let bytes: Vec<u8> = response.bytes().await?.into_iter().collect();
    program_file.write_all(&bytes)?;
    Ok(())
}

/// Program location.
pub enum Location {
    GitHub {
        program: String,
        repo: String,
        version: String,
    },
    URL {
        url: String,
    },
    Local {
        path: PathBuf,
    },
}

fn parse_gh_location(location: &str) -> Option<Location> {
    let (_, raw_gh_ref) = location.split_once("gh:")?;
    let (program, rest) = raw_gh_ref.split_once(':')?;
    let (repo, version) = rest.split_once('@')?;
    Some(Location::GitHub {
        program: program.to_string(),
        repo: repo.to_string(),
        version: version.to_string(),
    })
}

impl TryFrom<&str> for Location {
    type Error = anyhow::Error;

    fn try_from(location: &str) -> Result<Self> {
        if location.starts_with("gh:") {
            parse_gh_location(location).ok_or_else(|| {
                format_err!(
                    "invalid gh format: should look like `gh:smart_wallet:GokiProtocol/goki@0.5.2`"
                )
            })
        } else if location.starts_with("https://") || location.starts_with("http://") {
            Ok(Location::URL {
                url: location.to_string(),
            })
        } else {
            Ok(Location::Local {
                path: PathBuf::from(location),
            })
        }
    }
}

impl Location {
    /// Fetches the program file associated with the [Location].
    pub async fn fetch_program_file<W: Write>(self, program_file: &mut W) -> Result<()> {
        match self {
            Location::GitHub {
                program,
                repo,
                version,
            } => {
                let target = format!(
                    "https://github.com/{}/releases/download/v{}/{}.so",
                    repo, version, program
                );
                download_file(program_file, &target).await?;
            }
            Location::URL { url } => {
                download_file(program_file, &url).await?;
            }
            Location::Local { path } => {
                let mut file = File::open(&path)?;
                copy(&mut file, program_file)?;
            }
        };
        Ok(())
    }
}

/// Fetches a program from a location.
pub async fn fetch_program_file<W: Write>(program_file: &mut W, location_str: &str) -> Result<()> {
    let location = Location::try_from(location_str)?;
    location.fetch_program_file(program_file).await?;
    Ok(())
}
