use routes::RouteOutlet;
use view::nav;
use yew::prelude::*;
mod routes;
#[function_component(App)]
pub fn app() -> Html {
    html! {
            <RouteOutlet />
    }
}
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

