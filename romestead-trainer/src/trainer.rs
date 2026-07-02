use crate::profile::Profile;
use rand::Rng;

pub struct Trainer {
    pub game_version: String,
    pub active_patches: Vec<String>,
}

impl Trainer {
    pub fn new(version: &str) -> Self {
        Trainer {
            game_version: version.to_string(),
            active_patches: Vec::new(),
        }
    }

    pub fn apply_profile(&mut self, profile: &Profile) {
        for patch in &profile.patches {
            println!("Applying patch: {} (addr: {:X})", patch.name, patch.address);
            self.active_patches.push(patch.name.clone());
        }
    }

    pub fn randomize_stat(&self, base: u32, range: u32) -> u32 {
        let mut rng = rand::thread_rng();
        base + rng.gen_range(0..=range)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_trainer() {
        let t = Trainer::new("test");
        assert_eq!(t.game_version, "test");
        assert!(t.active_patches.is_empty());
    }

    #[test]
    fn test_apply_profile() {
        let mut t = Trainer::new("test");
        let p = Profile {
            name: "test_profile".into(),
            patches: vec![crate::profile::Patch {
                name: "health".into(),
                address: 0x1234,
                value: 100,
            }],
        };
        t.apply_profile(&p);
        assert_eq!(t.active_patches.len(), 1);
    }

    #[test]
    fn test_randomize_stat() {
        let t = Trainer::new("test");
        let result = t.randomize_stat(50, 10);
        assert!(result >= 50 && result <= 60);
    }
}