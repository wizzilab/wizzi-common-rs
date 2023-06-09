#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic_in_result_fn)]
#![deny(clippy::panic)]
#![deny(clippy::indexing_slicing)]

use std::sync::Weak;
use tokio::sync::{mpsc, Mutex};

pub async fn forward_to_multiple<T: Clone>(
    mut rx: mpsc::Receiver<T>,
    listeners: Weak<Mutex<Vec<mpsc::Sender<T>>>>,
) {
    let mut to_rm = vec![];
    while let Some(upload) = rx.recv().await {
        let listeners = match listeners.upgrade() {
            Some(listeners) => listeners,
            None => return,
        };
        let mut listeners = listeners.lock().await;
        for (i, listener) in listeners.iter().enumerate() {
            if listener.send(upload.clone()).await.is_err() {
                to_rm.push(i);
            }
        }

        for i in to_rm.drain(..).rev() {
            listeners.remove(i);
        }
    }
}

pub async fn forward_map_to_multiple<T, U: Clone, F: Fn(T) -> U>(
    mut rx: mpsc::Receiver<T>,
    map: F,
    listeners: Weak<Mutex<Vec<mpsc::Sender<U>>>>,
) {
    let mut to_rm = vec![];
    while let Some(upload) = rx.recv().await {
        let upload = map(upload);
        let listeners = match listeners.upgrade() {
            Some(listeners) => listeners,
            None => return,
        };
        let mut listeners = listeners.lock().await;
        for (i, listener) in listeners.iter().enumerate() {
            if listener.send(upload.clone()).await.is_err() {
                to_rm.push(i);
            }
        }

        for i in to_rm.drain(..).rev() {
            listeners.remove(i);
        }
    }
}
