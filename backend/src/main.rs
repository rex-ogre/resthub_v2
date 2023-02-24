use axum::{
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::{get, get_service},
    Json, Router,
};
use glob::glob;
use model::post::Post;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{io, net::SocketAddr};
use tower_http::cors::CorsLayer;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::util::SubscriberInitExt;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tokio::join!(serve(using_serve_dir_with_assets_fallback(), 3002),);
}

fn using_serve_dir_with_assets_fallback() -> Router {
    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));
    let serve_dir = get_service(serve_dir).handle_error(handle_error);

    Router::new()
        .route("/foo", get(|| async { "Hi from /foo" }))
        .route("/json", get(jsons))
        .layer(
            CorsLayer::new()
                .allow_origin("http://127.0.0.1:8080".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        )
        .fallback_service(serve_dir)
}
async fn jsons() -> Json<serde_json::Value> {
    tracing::debug!("開始掃檔案");
    let mut post_list: Vec<Post> = Vec::new();
    for entry in glob("../backend/assets/*.md").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                path.file_name().unwrap().to_str().unwrap();
                let markdown_input = std::fs::read_to_string(&path).unwrap();
                let parser = pulldown_cmark::Parser::new(&markdown_input);
                let mut html_output = String::new();
                pulldown_cmark::html::push_html(&mut html_output, parser);
                let post = Post::new(
                    &markdown_input,
                    &path.file_name().unwrap().to_str().unwrap(),
                );
                post_list.push(post.clone());
                tracing::debug!("{:?}", markdown_input);
                tracing::debug!("{:?}", post);
            }
            Err(e) => tracing::debug!("錯誤{:?}", e),
        }
    }
    Json(serde_json::json!(post_list))
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
}
