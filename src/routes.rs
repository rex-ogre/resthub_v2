use about::About;
use router::RootRoutes;
use std::rc::Rc;
use utils::theme_provider::{use_prefered_dark, Theme};
use yew::prelude::*;
use yew_router::prelude::*;
fn switch(routes: RootRoutes) -> Html {
    match routes {
        RootRoutes::About => html! { <About /> },
        _ => html! { {"error12324"} },
    }
}

#[function_component(RouteOutlet)]
pub fn route_outlet() -> Html {
    let ctx = use_reducer(|| Theme {
        dark_theme: use_prefered_dark(),
    });
    html! {
        <ContextProvider<UseReducerHandle<Theme>> context={ctx}>
        <BrowserRouter>
             <Switch<RootRoutes> render={switch} />
        </BrowserRouter>
        </ContextProvider<UseReducerHandle<Theme>>>
    }
}
