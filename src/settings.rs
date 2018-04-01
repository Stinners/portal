use std::path::PathBuf;

use resource::Resource;

lazy_static! {
    pub static ref SETTINGS: Settings = {
        let path = PathBuf::from("test_settings.json");
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
        Settings {
            state_file: PathBuf::from("./test_state.json"),
            database_file: PathBuf::from("./test_db.json"),
            database_size: 100,
        }
    }
}
