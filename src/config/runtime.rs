use tokio::runtime::{Builder, Runtime};

pub async fn build(cpus: usize) -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(cpus * 2)
        .max_blocking_threads(cpus * 2)
        .enable_all()
        .build()
        .unwrap()
}
