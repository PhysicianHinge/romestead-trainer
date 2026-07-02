use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct Patch {
    pub name: String,
    pub address: u64,
    pub value: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Profile {
    pub name: String,
    pub patches: Vec<Patch>,
}

impl Profile {
    pub fn load(path: &str) -> Result<Self, String> {
        let data = fs::read_to_string(path).map_err(|e| format!("Failed to read profile: {}", e))?;
        serde_json::from_str(&data).map_err(|e| format!("Failed to parse profile: {}", e))
    }

    pub fn save(&self, path: &str) -> Result<(), String> {
        let data = serde_json::to_string_pretty(self).map_err(|e| format!("Failed to serialize: {}", e))?;
        fs::write(path, data).map_err(|e| format!("Failed to write profile: {}", e))
    }
}

impl Default for Profile {
    fn default() -> Self {
        Profile {
            name: "default".into(),
            patches: vec![
                Patch { name: "infinite_health".into(), address: 0xDEAD, value: 9999 },
                Patch { name: "max_mana".into(), address: 0xBEEF, value: 500 },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_profile() {
        let p = Profile::default();
        assert_eq!(p.name, "default");
        assert_eq!(p.patches.len(), 2);
    }

    #[test]
    fn test_save_and_load() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_str().unwrap();
        
        let p = Profile::default();
        assert!(p.save(path).is_ok());
        
        let loaded = Profile::load(path).unwrap();
        assert_eq!(loaded.name, "default");
        assert_eq!(loaded.patches[0].name, "infinite_health");
    }
}