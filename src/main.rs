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

use std::env::current_dir;
use std::path::PathBuf;

use clap::{Arg, App};

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
                       .get_matches();

    // Load the database 
    let mut database = database::Database::fetch(&SETTINGS.database_file).unwrap();

    // ======= Dispatch based on the arguments ====== // 

    if let Some(jump) = args.value_of("Jump") {
        println!("{}", "Do Jump");
    }

    // This is essentially the else clause
    if let Some(step) = args.value_of("Step") {
        let step = PathBuf::from(step);
        let step = make_absolue(step);
        let _ = database::step(&mut database, &step);
    }

}

fn make_absolue(path: PathBuf) -> PathBuf {
    if path.is_relative() {
        current_dir().unwrap().join(path)
    } else {
        path
    }
}

