use crate::server::routes::v1::create_v1_routes;
use axum::Router;

pub mod v1;

pub fn create_routes() -> Router {
    Router::new().nest("/v1/", create_v1_routes())
}
