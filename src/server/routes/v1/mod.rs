use axum::{routing::get, Router};
use hello_world::hello_world_route;

pub mod hello_world;

pub fn create_v1_routes() -> Router {
    Router::new().route("/", get(hello_world_route()))
}
