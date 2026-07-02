mod trainer;
mod profile;

use trainer::Trainer;
use profile::Profile;

fn main() {
    let mut trainer = Trainer::new("Romestead v1.0");
    
    let profile = Profile::load("default.json").unwrap_or_else(|_| {
        println!("No existing profile found, creating default.");
        Profile::default()
    });

    trainer.apply_profile(&profile);
    
    println!("Trainer active. Press Ctrl+C to exit.");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}