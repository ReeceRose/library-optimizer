use axum::response::Html;

pub fn hello_world_route() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
