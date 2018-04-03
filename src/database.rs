use std::collections::VecDeque;
use std::env;
use std::io::{Result};
use std::path::PathBuf;

use resource::Resource;
use state::{update_for_move, STATE};
use settings::SETTINGS;
use jaro_winkler::jaro_winkler;

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

/* Search for a dirrectory in the database and return Some(PathBuf) if 
 * a match is found r None if not match is found, returns immediatly 
 * if an exact match is found 
 */
pub fn search(database: &Database, new_path: PathBuf) -> Option<PathBuf> {
    
    let mut match_value = 0.0;
    let mut match_index = 0;
    for (i, path) in database.iter().enumerate() {

        // This fails on invalid filenames
        // .file_name() returns none if the path terminates in ..
        let name = path.file_name().unwrap()
                       .to_str().unwrap();

        let similarity = jaro_winkler(new_path.to_str().unwrap(), name);

        if similarity > 0.99 {
            return Some(path.to_path_buf());
        } else if similarity > match_value {
            match_value = similarity;
            match_index = i;
        }

    }

    if match_value > 0.1 {
        Some(database[match_index].clone())
    } else { 
        None
    }
}

/* step is used to catually call cd to go somewhere, it also handles 
 * updating the database and state
 */
pub fn step(database: &mut Database, new_dir: PathBuf) -> Result<()> {

    // Update and save the database 
    let _ = add_value(database, &new_dir);
    let _ = database.dump(&SETTINGS.database_file);
    
    // Update and save settings
    let new_state = update_for_move(&STATE);
    let _ = new_state.dump(&SETTINGS.state_file);

    // Print the result to stdout for the shell to pick up
    print!("cd {}", new_dir.to_str().unwrap());

    Ok(())
}

        
pub fn jump(database: &mut Database, search_path: PathBuf) -> Result<()> {
    match search(database, search_path) {
        Some(path) => step(database, path),
        None => {
            println!("echo \"Could not find path\"");
            Ok(())
        }
    }
}

pub fn back(database: &mut Database) -> Result<()> {
    step(database, STATE.last_dir.clone())
}


