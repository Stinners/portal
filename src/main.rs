#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate serde;
extern crate serde_json;

mod database;
mod resource;
mod settings;
mod state;
mod jaro_winkler;

use std::env;
use std::path::PathBuf;

use clap::{Arg, App, ArgMatches};

use settings::SETTINGS;
use resource::Resource;

fn main() {
    let args = App::new("Portal")
                       .version("0.1")
                       .author("Chris Stinson")
                       .about("A wrapper around cd")
                       .arg(Arg::with_name("Step")
                            .value_name("STEP")
                            .help("Move to a directory like normal cd")
                            .takes_value(true))
                       .arg(Arg::with_name("Jump")
                            .short("j")
                            .long("jump")
                            .value_name("JUMP")
                            .help("Search for a directory and jump to it")
                            .takes_value(true))
                       .arg(Arg::with_name("Back")
                            .short("b")
                            .long("back")
                            .help("Return to the previous directory"))
                       .get_matches();

    // Load the database 
    let mut database = database::Database::fetch(&SETTINGS.database_file).unwrap();

    // ======= Dispatch based on the arguments ====== // 

    if let Some(jump) = args.value_of("Jump") {
        let jump = PathBuf::from(jump);
        let _ = database::jump(&mut database, jump);
    }

    if args.is_present("Back") {
        let _ = database::back(&mut database);
    }

    // This is essentially the else clause
    if let Some(step) = get_path(&args, "Step") {
        let step = PathBuf::from(step);
        let _ = database::step(&mut database, step);
    }



}

/* Extracts the path from the arguments and nomalizes it 
*/
fn get_path(args: &ArgMatches, value: &str) -> Option<PathBuf> {
    if let Some(path) = args.value_of(value) {
        normalize_path(PathBuf::from(path))
    } else {
        None
    }
}


// Makes a path satrting with '~' absolute
fn expand_home(path: PathBuf) -> PathBuf {
    let new_path: PathBuf = path.components().skip(1).collect();
    env::home_dir().unwrap().join(new_path)
}

/* Returns a existing dirrectory as the shortest absolute path 
 * for returns None if it can't find the dirrectory 
 */ 
pub fn normalize_path(mut path: PathBuf) -> Option<PathBuf> {
    // Check for tilde and expand out if nessescary 
    if path.starts_with("~") {
        path = expand_home(path)
    }

    // Check that the dirrectory exisits, for now just 
    // return None if it doesn't
    // TODO make this handle these case more intelligenty
    if !(path.exists()) {
        return None;
    }

    // cannonicalise the path and return Some(path)
    path.canonicalize().ok()
}


#[cfg(test)]
mod tests {

    use std::env;
    use std::path::PathBuf;
    use::database::normalize_path;

    #[test]
    fn dots() {
        let cannon_path = normalize_path(PathBuf::from("/etc/.././etc")).unwrap();
        assert_eq!(cannon_path, PathBuf::from("/etc"));
    }

    #[test]
    fn invalid_path() {
        let cannon_path = normalize_path(PathBuf::from("fkhaeldw/fgyakhdw"));
        assert_eq!(cannon_path, None);
    }

    #[test]
    fn relative_path() {
        let home_dir = env::home_dir().unwrap();
        let _ = env::set_current_dir(&home_dir);
        let cannon_dir = normalize_path(PathBuf::from("Documents")).unwrap();
        let abs_dir = home_dir.join(PathBuf::from("Documents"));
        assert_eq!(cannon_dir, abs_dir);
    }

    #[test]
    fn home_path() {
        let cannon_path = normalize_path(PathBuf::from("~/Documents")).unwrap();
        let abs_path = env::home_dir().unwrap().join(PathBuf::from("Documents"));
        assert_eq!(cannon_path, abs_path);
    }

}
