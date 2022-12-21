use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum RootRoutes {
    #[at("/")]
    Root,
    #[at("/about")]
    About,
    #[at("/search")]
    Search,
    // Compatible with https://github.com/jetli/awesome-yew
}
