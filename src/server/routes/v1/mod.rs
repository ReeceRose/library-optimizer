use crate::server::routes::v1::hello_world::hello_world_route;

use axum::{routing::get, Router};

pub mod hello_world;

pub fn create_v1_routes() -> Router {
    Router::new().route("/", get(hello_world_route()))
}
