mod config;
mod database;
mod server;

use notify::{recommended_watcher, RecursiveMode, Watcher};
use std::env;
use std::path::Path;
use std::sync::{mpsc::channel, Arc, Mutex};
use tracing::{debug, error, info};

const DB_URL: &str = "sqlite://library-optimizer.db";

#[tokio::main]
async fn main() {
    config::logging::init();
    let version = option_env!("BUILD_HASH").unwrap_or("dev-build");

    info!("Running library optimizer: {}", version);
    let cpus = num_cpus::get();
    debug!("Current CPU count: {}", cpus);

    let _db = database::Database::new(DB_URL).await;

    let rt = config::runtime::build(cpus).await;

    let tv_dir = env::var("TV_DIR").unwrap_or(String::from("/data/tv"));
    let movies_dir = env::var("MOVIE_DIR").unwrap_or(String::from("/data/media"));
    let directories_to_watch = [tv_dir, movies_dir];

    // File notification channel
    let (sender, receiver) = channel();

    let receiver_arc = Arc::new(Mutex::new(receiver));
    let receiver_arc_clone = receiver_arc.clone();
    rt.spawn(async move {
        let receiver_locked = receiver_arc_clone.lock().unwrap();
        while let Ok(event) = receiver_locked.recv() {
            info!("Received file event: {:?}", event);
        }
    });

    let sender_arc = Arc::new(Mutex::new(sender));
    for dir in directories_to_watch {
        let sender_arc_clone = sender_arc.clone();
        let dir_clone = dir.to_string();

        rt.spawn(async move {
            let (watcher_sender, watcher_receiver) = channel();
            let mut file_watcher = recommended_watcher(watcher_sender).unwrap();

            file_watcher
                .watch(Path::new(&dir_clone), RecursiveMode::Recursive)
                .unwrap();

            loop {
                match watcher_receiver.recv() {
                    Ok(event) => {
                        sender_arc_clone.lock().unwrap().send(event).unwrap();
                    }
                    Err(error) => error!("Watch error: {:?}", error),
                }
            }
        });
    }

    server::serve().await;
}
