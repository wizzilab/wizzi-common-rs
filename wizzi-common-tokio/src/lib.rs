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
