use reqwest::Client;

use crate::request::{
    artstation_api::ArtStationCsrf, da_api::DeviantArtAuth, request_api::RequestAPI,
};

use super::workspace::Workspace;
use std::sync::{Arc, Weak};
use tauri::async_runtime::Mutex;

#[derive(Debug, Default)]
pub struct State {
    pub workspace: Option<Workspace>,
}

impl State {
    pub fn set_workspace(&mut self, ws: Workspace) {
        self.workspace = Some(ws);
    }
}

pub struct TauriState {
    pub state: Mutex<State>,
    pub reqwest_client: Client,
    pub artstation_tokens: Mutex<Option<ArtStationCsrf>>,
    pub da_auth: Mutex<Option<DeviantArtAuth>>,

    pub da_api: Mutex<Option<Arc<RequestAPI>>>,
}

impl TauriState {
    pub fn new(wnd: tauri::Window) -> Arc<Self> {
        let mut res = Arc::new(Self {
            state: Mutex::new(State::default()),
            reqwest_client: Client::new(),
            artstation_tokens: Mutex::new(None),
            da_auth: Mutex::new(None),
            // da_api: Mutex::new(RequestAPI::new(5, state))
            da_api: Mutex::new(None),
        });
        // res.da_api = Some(Mutex::new(RequestAPI::new(5, Arc::downgrade(&res), wnd)));
        {
            let mut l = res.da_api.blocking_lock();
            *l = Some(RequestAPI::new(5, Arc::downgrade(&res), wnd));
        }
        res
    }
}
