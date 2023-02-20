use axum::{
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::{get, get_service},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use glob::glob;
use std::{io, net::SocketAddr};
use tower_http::cors::{ CorsLayer};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{util::SubscriberInitExt};
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tokio::join!(serve(using_serve_dir_with_assets_fallback(), 3002),);
}

fn using_serve_dir_with_assets_fallback() -> Router {
    // for example `ServeDir` allows setting a fallback if an asset is not found
    // so with this `GET /assets/doesnt-exist.jpg` will return `index.html`
    // rather than a 404
    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));
    let serve_dir = get_service(serve_dir).handle_error(handle_error);

    Router::new()
        .route("/foo", get(|| async { "Hi from /foo" }))
        .nest_service("/assets", serve_dir.clone())
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
    for entry in glob("../backend/assets/*.md").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let markdown_input = std::fs::read_to_string(&path).unwrap();
                let parser = pulldown_cmark::Parser::new(&markdown_input);
                let mut html_output = String::new();
                pulldown_cmark::html::push_html(&mut html_output, parser);
                let title_index = vec![html_output.find(">").unwrap() ,html_output.find("</h1>").unwrap()];
                tracing::debug!("{:?}", path.display());
                tracing::debug!("{:?}",title_index);
                tracing::debug!("{:?}",&html_output);
                tracing::debug!("{:?}",&html_output[title_index[0]..title_index[1]]);
            }
            Err(e) => tracing::debug!("{:?}", e),
        }
    }
    Json(serde_json::json!({"hello":"axum.rs"}))
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

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    filename: String,
    date: String,
    title: String,
    info: String
}
