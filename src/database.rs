
use std::env;
use std::io::Result;
use std::collections::VecDeque;
use std::path::PathBuf;

use resource::Resource;
use settings::SETTINGS;

pub type Database = VecDeque<PathBuf>;

impl Resource for Database {

    fn initial_state() -> Database {
        let mut database = VecDeque::new();
        database.push_front(env::current_dir().unwrap());
        database
    }

}

pub fn add_value(database: &mut Database, path: &PathBuf) {
    // First check if the value is in the database already
    let index = database.iter().position(|db_path| db_path == path);

    match index {
        // If index is in the database move it to the front
        Some(idx) => {
            database.remove(idx);
            database.push_front(path.clone());
        }
        // Else push it to the front and check if the database 
        // is above capacity
        None => {
            database.push_front(path.clone());
            while database.len() > SETTINGS.database_size {
                let _ = database.pop_back();
            }
        }
    }
}

pub fn visit(database: &mut Database, path: &PathBuf) -> Result<()> {
    
   env::set_current_dir(path)?;

   //TODO make this update state as well 

   add_value(database, &env::current_dir()?);

   Ok(())

}
