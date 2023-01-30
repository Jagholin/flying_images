use crossbeam::channel::Sender;
use crossbeam::atomic::AtomicCell;
use reqwest::Url;
use serde::Serialize;
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Weak}, path::PathBuf,
};
use tauri::async_runtime::Mutex;

use crate::state::state::TauriState;

// to remind:  let f: MyFunc = Box::new(|a| Box::pin(add_two(a)));
type RequestRunner = Box<
    dyn FnOnce(
            Arc<TauriState>,
            tauri::Window,
        ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send>>
        + Send,
>;

pub enum AsyncRequest {
    ImageDownload(Url, PathBuf),
}

/// Structure that saves statistics about image download/search
/// requests to the given API and helps making the crawling proccess
/// more fair
///
/// When the user issues a request to bulk download a list of images
/// or other assets, it first arrives as a command in the tauri's command
/// API. The command adds the requests for download to the API queue which gets a tracking ID and
/// returns a "pending" status response with the tracking ID to the front-end.
///
/// The front-end now has an option to wait for the tasks in the
/// queue to finish or issue a cancellation request(in form of another Tauri command).
/// Cancelled requests will be removed from the queue, and already running requests will
/// be run to completion.
pub struct RequestAPI {
    // last_request_at: Mutex<Option<std::time::Instant>>,
    last_request_at: AtomicCell<Option<std::time::Instant>>,
    requests_made: u32,
    request_queue: Sender<(RequestRunner, u32)>,
    last_task_id: u32,
}

#[derive(Debug, Serialize, Clone)]
pub struct TaskResult {
    task_id: u32,
    task_result: Result<String, String>,
}

impl RequestAPI {
    /// creates RequestAPI object and starts tasks for tauri async backend to
    /// retrieve requests from the queue and execute them in parallel
    pub fn new(
        parallel_threads: usize,
        state: Weak<TauriState>,
        window: tauri::Window,
    ) -> Self {
        let (send, recv) =
            crossbeam::channel::bounded::<(RequestRunner, u32)>(16 * parallel_threads);
        println!("atomic cell operations are lock free: {}", AtomicCell::<std::time::Instant>::is_lock_free());
        let res = Self {
            last_request_at: AtomicCell::new(None),
            requests_made: 0,
            request_queue: send,
            last_task_id: 0,
        };
        for _ in 0..parallel_threads {
            let local_recv = recv.clone();
            let local_state = state.clone();
            let local_wnd = window.clone();
            let local_self = res.clone();
            tauri::async_runtime::spawn_blocking( move || {
                loop {
                    let loop_wnd = local_wnd.clone();
                    let loop_state = local_state.clone();
                    let loop_state = match loop_state.upgrade() {
                        Some(x) => x,
                        None => break,
                    };
                    println!("start listening to tasks...");
                    let func = local_recv.recv();
                    match func {
                        Ok(f) => {
                            local_self.last_request_at.store(Some(std::time::Instant::now()));
                            let result = tauri::async_runtime::block_on(f.0(loop_state, loop_wnd.clone()));
                            // let result = .await;
                            
                            if loop_wnd
                                .emit(
                                    "task_finished",
                                    TaskResult {
                                        task_id: f.1,
                                        task_result: result,
                                    },
                                )
                                .is_err()
                            {
                                println!("window emit returned an Error, stopping task...");
                                break;
                            };
                        }
                        Err(_e) => {
                            // channel is empty and disconnected, so just break out of the loop
                            println!("channel disconnected, stopping task...");
                            break;
                        }
                    }
                }
            });
        }
        res
    }

    /// Adds a task to be executed later with tauri async API
    /// each task receives a task id
    pub fn add_to_queue(&mut self, req: RequestRunner) -> u32 {
        // we dont expect an error here - channel shouldnt be disconnected from the sender side.
        let task_id = self.last_task_id + 1;
        self.last_task_id = task_id;

        self.requests_made += 1;
        self.request_queue.send((req, task_id)).unwrap();
        task_id
    }

    pub fn pending_requests(&self) -> usize {
        self.request_queue.len()
    }
}
