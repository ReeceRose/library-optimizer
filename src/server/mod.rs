pub mod routes;

use crate::server::routes::create_routes;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing::info;

pub async fn serve() {
    let app = Router::new()
        .nest("/", create_routes())
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());

    let tcp_listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    info!("Starting server on 8080");
    axum::serve(tcp_listener, app).await.unwrap();
}
