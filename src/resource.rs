use std::fs::File;
use std::io::{Read, Result, Write};
use std::path::PathBuf;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json;

pub trait Resource: Serialize + DeserializeOwned {
    fn initial_state() -> Self;

    // Store a resource in a file
    fn dump(&self, path: &PathBuf) -> Result<()> {
        let mut file = File::create(path)?;
        let data = serde_json::to_string_pretty(self).unwrap();
        file.write(&data.as_bytes())?;
        Ok(())
    }

    // Sets the inital state of a resource and stores
    // it to file
    fn init(path: &PathBuf) -> Result<()> {
        let initial = Self::initial_state();
        let _ = Self::dump(&initial, path);
        Ok(())
    }

    // Get a resource from a file, initializing it if nessecary
    fn fetch(path: &PathBuf) -> Result<Self> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                let _ = Self::init(path);
                File::open(path).unwrap()
            }
        };
        let mut data = String::new();
        let _ = file.read_to_string(&mut data);
        let result = serde_json::from_str(&data).unwrap();
        Ok(result)
    }
}
