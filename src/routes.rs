use about::About;
use router::RootRoutes;
use yew::prelude::*;
use yew_router::prelude::*;
fn switch(routes: RootRoutes) -> Html {
    match routes {
        RootRoutes::About => html! { <About /> },
        _ => html! { {"error"} },
    }
}

#[function_component(RouteOutlet)]
pub fn route_outlet() -> Html {
    html! {
        <BrowserRouter>
             <Switch<RootRoutes> render={switch} />
        </BrowserRouter>
    }
}
