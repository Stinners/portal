use std::env;
use std::path::PathBuf;

use resource::Resource;

lazy_static! {
    pub static ref SETTINGS: Settings = {
        let home_dir = env::home_dir().unwrap();
        let path = home_dir.join(PathBuf::from(".config/portal/settings.json"));
        Settings::fetch(&path).unwrap()
    };
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub state_file: PathBuf,
    pub database_file: PathBuf,
    pub database_size: usize,
}

impl Resource for Settings {
    fn initial_state() -> Settings {
        let home_dir = env::home_dir().unwrap();
        Settings {
            state_file: home_dir.join(PathBuf::from(".config/portal/state.json")),
            database_file: home_dir.join(PathBuf::from(".config/portal/database.json")),
            database_size: 100,
        }
    }
}
