use crate::History;
use async_std::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum LoadError {
    File,
    Format,
}

#[derive(Debug, Clone)]
pub enum SaveError {
    File,
    Write,
    Format,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedState {
    pub input_val: String,
    pub history: History,
    pub at: usize,
}

#[cfg(not(target_arch = "wasm32"))]
impl SavedState {
    fn path() -> std::path::PathBuf {
        let mut path = if let Some(project_dirs) =
            directories_next::ProjectDirs::from("rs", "Beacon", "Data")
        {
            project_dirs.data_dir().into()
        } else {
            std::env::current_dir().unwrap_or_default()
        };
        path.push("history.json");
        path
    }

    pub async fn load() -> Result<SavedState, LoadError> {
        let mut contents = String::new();
        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::File)?;
        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::File)?;
        serde_json::from_str(&contents).map_err(|_| LoadError::Format)
    }

    pub async fn save(self) -> Result<(), SaveError> {
        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::Format)?;
        let path = Self::path();
        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::File)?;
        }
        {
            let mut file = async_std::fs::File::create(path)
                .await
                .map_err(|_| SaveError::File)?;

            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::Write)?;
        }
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;
        Ok(())
    }
}
