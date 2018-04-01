

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate serde;
extern crate serde_json;

mod database;
mod state;
mod resource;
mod settings;

use resource::Resource;

use state::STATE;
use settings::SETTINGS;

fn main() {
   let db = database::Database::fetch(&SETTINGS.database_file); 
   println!("{:?}", db);
   println!("{:?}", STATE.last_dir);
}
