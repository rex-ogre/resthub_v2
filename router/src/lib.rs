use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum RootRoutes {
    #[at("/")]
    Root,
    #[at("/about")]
    About,
    #[at("/search")]
    Search,
    #[at("/content/")]
    Content,
    #[at("/post/:id")]
    Post { id: String },
}
