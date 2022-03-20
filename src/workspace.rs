use anchor_client::Cluster;
use anyhow::{format_err, Result};
use std::{
    path::PathBuf,
    process::{Command, Output},
};

use crate::{
    solana_cmd::new_solana_cmd,
    utils::{exec_command, get_cluster_url},
};

#[derive(Clone, Debug, Default)]
pub struct Workspace {
    pub path: PathBuf,
}

impl Workspace {
    pub fn deployer_dir(&self) -> PathBuf {
        self.path.join("deployers/")
    }

    pub fn get_deployer_kp_path(&self, cluster: &Cluster) -> PathBuf {
        let deployer_dir = self.deployer_dir();
        deployer_dir.as_path().join(format!("{}.json", cluster))
    }

    pub fn get_deployer_kp_path_if_exists(&self, cluster: &Cluster) -> Result<PathBuf> {
        let deployer_dir = self.deployer_dir();
        if !deployer_dir.exists() {
            return Err(format_err!(
                "{} does not exist; you may need to run `goki init`",
                deployer_dir.display()
            ));
        }
        let deployer_kp_path = deployer_dir.as_path().join(format!("{}.json", cluster));
        if !deployer_kp_path.exists() {
            return Err(format_err!(
                "Deployer not found at {}; you may need to run `goki init`",
                deployer_kp_path.display()
            ));
        }
        Ok(deployer_kp_path)
    }

    pub fn add_cluster_args(&self, command: &mut Command, cluster: &Cluster) -> Result<()> {
        let kp_path = self.get_deployer_kp_path_if_exists(cluster)?;
        command
            .args(["--url", &get_cluster_url(cluster)?, "--keypair"])
            .arg(kp_path);
        Ok(())
    }

    pub fn exec_deployer_command<F>(&self, cluster: &Cluster, mut builder: F) -> Result<Output>
    where
        F: FnMut(&mut Command) -> Result<()>,
    {
        let cmd = &mut new_solana_cmd();
        self.add_cluster_args(cmd, cluster)?;
        builder(cmd)?;
        exec_command(cmd)
    }
}
