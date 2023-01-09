use super::workspace::Workspace;
use std::sync::Mutex;

#[derive(Debug, Default)]
pub struct State {
    pub workspace: Option<Workspace>,
}

impl State {
    pub fn set_workspace(&mut self, ws: Workspace) {
        self.workspace = Some(ws);
    }
}

#[derive(Debug)]
pub struct ManagedState {
    pub state: Mutex<State>,
}

impl ManagedState {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(State::default()),
        }
    }
}
