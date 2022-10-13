use crate::PathBuf;

use anyhow::{anyhow, Context, Result};
use directories::ProjectDirs;

use std::fs;

pub struct Session {
    api_key: String,
}

impl Session {
    pub fn key(self) -> String {
        self.api_key
    }

    pub fn load() -> Result<Self> {
        Ok(Self {
            api_key: fs::read_to_string(path()?)
                .context("Couldn't load api token. Try `pmis auth` first")?
                .trim()
                .into(),
        })
    }
    pub fn save(key: String) -> Result<()> {
        let path = path()?;
        fs::create_dir_all(
            path.parent()
                .context("Couldn't determine api key directory")?,
        )?;
        fs::write(&path, key).context("Couldn't write api key file")?;
        Ok(())
    }
}

fn path() -> Result<PathBuf> {
    Ok(ProjectDirs::from("me", "misterio", "pmis")
        .ok_or(anyhow!("Couldn't determine home directory"))?
        .config_dir()
        .to_owned()
        .join("api_key"))
}
