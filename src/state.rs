use std::env;
use std::path::PathBuf;

use resource::Resource;
use settings::SETTINGS;

lazy_static! {
    pub static ref STATE: State = State::fetch(&SETTINGS.state_file).unwrap();
}

#[derive(Clone, Serialize, Deserialize)]
pub struct State {
    pub last_dir: PathBuf,
}

impl Resource for State {
    fn initial_state() -> State {
        State {
            last_dir: env::current_dir().unwrap(),
        }
    }
}

pub fn update_for_move(state: &State) -> State {
    let mut new_state = (*state).clone();
    new_state.last_dir = env::current_dir().unwrap();
    new_state
}
