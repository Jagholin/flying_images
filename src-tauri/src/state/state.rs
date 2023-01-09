use reqwest::Client;

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
    pub reqwest_client: Client,
}

impl ManagedState {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(State::default()),
            reqwest_client: Client::new()
        }
    }
}
