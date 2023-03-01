use about::About;
use content::Content;
use home::Home;
use router::RootRoutes;
use search::Search;
use std::rc::Rc;
use utils::theme_provider::{get_theme_from_storage, Theme};
use yew::prelude::*;
use yew_router::prelude::*;
fn switch(routes: RootRoutes) -> Html {
    match routes {
        RootRoutes::About => html! { <About /> },
        RootRoutes::Root => html! {<Home/>},
        RootRoutes::Search => html! {<Search/>},
        _ => html! {<Content/>},
    }
}

#[function_component(RouteOutlet)]
pub fn route_outlet() -> Html {
    let ctx = use_reducer(|| get_theme_from_storage());
    html! {
        <ContextProvider<UseReducerHandle<Theme>> context={ctx}>
        <BrowserRouter>
             <Switch<RootRoutes> render={switch} />
        </BrowserRouter>
        </ContextProvider<UseReducerHandle<Theme>>>
    }
}
