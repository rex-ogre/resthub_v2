use about::About;
use content::Content;
use home::Home;
use router::RootRoutes;
use search::Search;
use utils::language_provider::{get_lang_from_storage, LangRrovider};
use utils::theme_provider::{get_theme_from_storage, Theme};
use yew::prelude::*;
use yew_router::prelude::*;
fn switch(routes: RootRoutes) -> Html {
    match routes {
        RootRoutes::About => html! { <About /> },
        RootRoutes::Root => html! {<Home/>},
        RootRoutes::Search => html! {<Search/>},
        RootRoutes::Post { id } => html! {<Content/>},
    }
}

#[function_component(RouteOutlet)]
pub fn route_outlet() -> Html {
    let ctx = use_reducer(|| get_theme_from_storage());
    let lang_ctx = use_reducer(|| get_lang_from_storage());
    html! {

        <ContextProvider<UseReducerHandle<LangRrovider>> context={lang_ctx}>
        <ContextProvider<UseReducerHandle<Theme>> context={ctx}>
        <BrowserRouter>
             <Switch<RootRoutes> render={switch} />
        </BrowserRouter>
        </ContextProvider<UseReducerHandle<Theme>>>
        </ContextProvider<UseReducerHandle<LangRrovider>>>
    }
}
