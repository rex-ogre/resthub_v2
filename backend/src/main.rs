use axum::{
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::{get, get_service},
    Json, Router,
};
use glob::glob;
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
                let title_index = vec![
                    html_output.find(">").unwrap() + 1,
                    html_output.find("</h1>").unwrap(),
                ];
                let data_index = vec![
                    html_output.find("<h6>").unwrap() + 4,
                    html_output.find("</h6>").unwrap(),
                ];
                let content_index = vec![html_output.find("<hr />").unwrap(), html_output.len()];
                let content_second_index = html_output[content_index[0] + 5..content_index[1]]
                    .find("<hr />")
                    .to_owned()
                    .unwrap();

                let content_slice = &html_output[content_index[0]..content_second_index]
                    .replace("<p>", "")
                    .replace("<hr />", "")
                    .replace("\n", "")
                    .replace("</p>", "")
                    .replace("<em>", "")
                    .replace("</em>", "")
                    .replace("<strong>", "")
                    .replace("</strong>", "")
                    .replace("<code>", "")
                    .replace("</code>", "")
                    .replace("<pre>", "")
                    .replace("</pre>", "")
                    .replace("<blockquote>", "")
                    .replace("</blockquote>", "")
                    .replace("<h1>", "")
                    .replace("</h1>", "")
                    .replace("<h2>", "")
                    .replace("</h2>", "")
                    .replace("<h3>", "")
                    .replace("</h3>", "")
                    .replace("<h4>", "")
                    .replace("</h4>", "")
                    .replace("<h5>", "")
                    .replace("</h5>", "")
                    .replace("<h6>", "")
                    .replace("</h6>", "");

                tracing::debug!("{:?}", path.display());
                tracing::debug!("{:?}", &html_output);
                //tracing::debug!("{:?}", &html_output[content_index[0]..content_index[1]]);
                tracing::debug!("這是slice{:?}", content_slice);
            }
            Err(e) => tracing::debug!("錯誤{:?}", e),
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
    info: String,
}
