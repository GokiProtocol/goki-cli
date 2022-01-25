use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::string::ToString;

pub struct WithPath<T> {
    inner: T,
    path: PathBuf,
}

impl<T> WithPath<T> {
    pub fn new(inner: T, path: PathBuf) -> Self {
        Self { inner, path }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> std::convert::AsRef<T> for WithPath<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T> std::ops::Deref for WithPath<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> std::ops::DerefMut for WithPath<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub rpc_endpoints: RPC,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RPC {
    pub mainnet: String,
    pub devnet: String,
    pub testnet: String,
    pub localnet: String,
    pub debug: String,
}

impl Default for RPC {
    fn default() -> Self {
        Self {
            mainnet: "https://api.mainnet-beta.solana.com".to_string(),
            devnet: "https://api.devnet.solana.com".to_string(),
            testnet: "https://api.testnet.solana.com".to_string(),
            localnet: "http://127.0.0.1:8899".to_string(),
            debug: "http://34.90.18.145:8899".to_string(),
        }
    }
}

impl Config {
    // Climbs each parent directory until we find an Goki.toml.
    pub fn discover() -> Result<Option<WithPath<Config>>> {
        let _cwd = std::env::current_dir()?;
        let mut cwd_opt = Some(_cwd.as_path());

        while let Some(cwd) = cwd_opt {
            for f in fs::read_dir(cwd)? {
                let p = f?.path();
                if let Some(filename) = p.file_name() {
                    if filename.to_str() == Some("Goki.toml") {
                        let cfg = Config::from_path(&p)?;
                        return Ok(Some(WithPath::new(cfg, p)));
                    }
                }
            }

            cwd_opt = cwd.parent();
        }

        Ok(None)
    }

    fn from_path(p: impl AsRef<Path>) -> Result<Self> {
        let mut cfg_file = File::open(&p)?;
        let mut cfg_contents = String::new();
        cfg_file.read_to_string(&mut cfg_contents)?;
        let cfg = cfg_contents.parse()?;

        Ok(cfg)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct _Config {
    rpc_endpoints: Option<RPC>,
}

impl ToString for Config {
    fn to_string(&self) -> String {
        let cfg = _Config {
            rpc_endpoints: Some(RPC {
                ..self.rpc_endpoints.clone()
            }),
        };

        toml::to_string(&cfg).expect("Must be well formed")
    }
}

impl FromStr for Config {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cfg: _Config = toml::from_str(s)
            .map_err(|e| anyhow::format_err!("Unable to deserialize config: {}", e.to_string()))?;
        Ok(Config {
            rpc_endpoints: cfg.rpc_endpoints.unwrap_or_default(),
        })
    }
}
