use reqwest::Client;

use crate::request::{da_api::DeviantArtAuth, artstation_api::ArtstationCsrf};

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
pub struct TauriState {
    pub state: Mutex<State>,
    pub reqwest_client: Client,
    pub artstation_tokens: Mutex<Option<ArtstationCsrf>>,
    pub da_auth: Mutex<Option<DeviantArtAuth>>,
}

impl TauriState {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(State::default()),
            reqwest_client: Client::new(),
            artstation_tokens: Mutex::new(None),
            da_auth: Mutex::new(None),
        }
    }
}
