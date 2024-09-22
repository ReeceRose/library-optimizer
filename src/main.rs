use axum::{response::Html, routing::get, Router};
use notify::{recommended_watcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::{mpsc, Arc, Mutex};
use tokio::net::TcpListener;
use tokio::runtime::Builder;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "library_optimizer=debug,axum=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let version = option_env!("BUILD_HASH").unwrap_or("dev-build");

    info!("Running library optimizer: {}", version);
    let cpus = num_cpus::get();
    debug!("CPUS: {}", cpus);

    let rt = Builder::new_multi_thread()
        .worker_threads(cpus * 2) // Fewer worker threads for handling async tasks
        .max_blocking_threads(cpus * 2) // More blocking threads for handling long-running blocking operations
        .enable_all()
        .build()
        .unwrap();
    let directories_to_watch = ["./data/tv", "./data/movies"];

    // Create a channel for communication between threads
    let (sender, receiver) = mpsc::channel();

    // Spawn a Tokio task to receive and print messages from the channel
    let receiver_arc = Arc::new(Mutex::new(receiver));
    let receiver_arc_clone = receiver_arc.clone();
    rt.spawn(async move {
        let receiver_locked = receiver_arc_clone.lock().unwrap();
        while let Ok(event) = receiver_locked.recv() {
            info!("Received file event: {:?}", event);
        }
    });

    // Spawn a Tokio task for each directory to monitor file changes and send messages to the channel
    let sender_arc = Arc::new(Mutex::new(sender));
    for dir in directories_to_watch {
        let sender_arc_clone = sender_arc.clone();
        let dir_clone = dir.to_string();

        rt.spawn(async move {
            // Create a watcher with a debounce delay
            let (watcher_sender, watcher_receiver) = mpsc::channel();
            let mut file_watcher = recommended_watcher(watcher_sender).unwrap();

            // Start watching the directory
            file_watcher
                .watch(Path::new(&dir_clone), RecursiveMode::Recursive)
                .unwrap();

            // Handle file system events
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

    // Set up the Axum server
    let app = Router::new()
        .route("/", get(handle_request))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());
    let tcp_listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!("Starting server on 8080");
    axum::serve(tcp_listener, app).await.unwrap();
}

async fn handle_request() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
