use super::*;

use crate::invoker::DetachedPreview;

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{spawn, JoinHandle};

#[derive(Debug)]
pub struct QueueState {
    queue: VecDeque<DetachedPreview>,
    done: bool,
}

pub struct ParRun {
    state: Arc<Mutex<QueueState>>,
    cond: Arc<Condvar>,
    handles: Vec<JoinHandle<()>>,
}

impl ParRun {
    pub fn new(cores: u16) -> ParRun {
        let state = QueueState {
            queue: VecDeque::new(),
            done: false,
        };

        let state = Arc::new(Mutex::new(state));
        let cond = Arc::new(Condvar::new());
        let mut handles = Vec::new();

        for _ in 0..cores {
            let state = state.clone();
            let cond = cond.clone();

            handles.push(spawn(move || loop {
                let preview = {
                    let mut lock = state.lock().unwrap();

                    match lock.queue.pop_front() {
                        Some(preview) => preview,

                        None => {
                            if lock.done {
                                break;
                            } else {
                                let mut l = cond.wait(lock).unwrap();
                                match l.queue.pop_front() {
                                    Some(preview) => preview,
                                    None => {
                                        if l.done {
                                            break;
                                        } else {
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                };

                preview.run().unwrap();
            }));
        }

        ParRun {
            state,
            cond,
            handles,
        }
    }
}

impl Process for ParRun {
    fn process(&self, preview: &Preview) {
        let mut state = self.state.lock().unwrap();

        state.queue.push_back(preview.detach());

        self.cond.notify_one();
    }

    fn finalize(self) {
        {
            let mut state = self.state.lock().unwrap();

            state.done = true;

            self.cond.notify_all();
        }

        for handle in self.handles {
            handle.join().unwrap();
        }
    }
}
