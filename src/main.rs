//use routes::RouteOutlet;
//use view::nav;
//use yew::prelude::*;
//mod routes;
//#[function_component(App)]
//pub fn app() -> Html {
//    html! {
//            <RouteOutlet />
//    }
//}
//fn main() {
//    wasm_logger::init(wasm_logger::Config::default());
//    yew::Renderer::<App>::new().render();
//}
use axum::{
    body::Body,
    handler::HandlerWithoutStateExt,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use axum_extra::routing::SpaRouter;
use std::{io, net::SocketAddr};
use tower::ServiceExt;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_static_file_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tokio::join!(
        serve(using_serve_dir(), 3001),
    );
}

fn using_serve_dir() -> Router {
    // `SpaRouter` is just a convenient wrapper around `ServeDir`
    //
    // You can use `ServeDir` directly to further customize your setup
    let serve_dir = get_service(ServeDir::new("post")).handle_error(handle_error);

    Router::new()
        .route("/foo", get(|| async { "Hi from /foo" }))
        .nest_service("/../post", serve_dir.clone())
        .fallback_service(serve_dir)
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
