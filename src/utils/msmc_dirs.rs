use std::{fs, process};
use std::path::{PathBuf};
use lazy_static::lazy_static;

const VAR_DIR: &str = "/var/lib/msmc";

pub struct MsmcDirs {
    pub path: PathBuf,
}

impl MsmcDirs {
    pub fn get_var() -> Self {
        let mut path = PathBuf::new();
        path.push(VAR_DIR);

        Self { path }
    }
    
    pub fn get_instances() -> Self {
        let mut msmc_dirs = MsmcDirs::get_var();
        msmc_dirs.path.push("minecraft_instances");

        msmc_dirs
    }
    
    pub fn get_instance(name: &str) -> Self {
        let mut msmc_dirs = MsmcDirs::get_instances();
        msmc_dirs.path.push(name);

        msmc_dirs
    }
    
    pub fn ensure_exists(&self) -> anyhow::Result<()> {
        if !self.path.exists() {
            fs::create_dir_all(&self.path)?;
        }
        
        Ok(())
    }
    
    pub fn delete(&self) -> anyhow::Result<()> {
        fs::remove_dir_all(&self.path)?;
        
        Ok(())
    }
}
