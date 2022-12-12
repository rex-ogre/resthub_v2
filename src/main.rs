use patternfly_yew::*;
use view::nav;
use yew::prelude::*;
#[function_component]
fn App() -> Html {
   

    html! {
        <div>
            <nav::Nav/>
        </div>
    }
}
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
