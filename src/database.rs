use std::{error::Error, fs, path::PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Database {
    pub balance: i32,
    pub total_spins: i32,
}

// TODO add a display command to use for `rspin stats`
impl Default for Database {
    fn default() -> Self {
        Self {
            balance: 1000,
            total_spins: 0,
        }
    }
}

impl Database {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let path = Self::get_path()?;

        if !path.exists() {
            let default_data = Self::default();
            default_data.save()?;
            return Ok(default_data);
        }

        let file_str = fs::read_to_string(path)?;
        let data: Self = toml::from_str(&file_str)?;
        Ok(data)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let path = Self::get_path()?;
        let toml_str = toml::to_string(self)?;
        fs::write(path, toml_str)?;
        Ok(())
    }

    pub fn get_path() -> Result<PathBuf, Box<dyn Error>> {
        let proj_dirs = ProjectDirs::from("com", "rspin-database", "rspin")
            .ok_or("Could not determine home directory")?;

        let data_dir = proj_dirs.data_dir();
        if !data_dir.exists() {
            fs::create_dir_all(data_dir)?;
        }

        Ok(data_dir.join("state.toml"))
    }
}
