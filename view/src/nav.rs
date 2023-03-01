use router::RootRoutes;
use utils::language_provider::*;
use utils::theme_provider::{self, Theme};
use yew::prelude::*;
use yew_router::{prelude::*, switch};
#[function_component]
pub fn RhNav() -> Html {
    let dark_state = use_context::<UseReducerHandle<Theme>>().unwrap();
    let switch_state: bool = dark_state.dark_theme;
    let theme_onclik: Callback<MouseEvent> = {
        Callback::from(move |_| {
            dark_state.dispatch(!dark_state.dark_theme);
            theme_provider::set_theme(!dark_state.dark_theme);
            // theme_provider::mount_on_theme_dom(dark_state.dark_theme);
        })
    };
    let lang_state = use_context::<UseReducerHandle<LangRrovider>>().unwrap();
    let switch_lang = lang_state.is_eng;
    let lang_onclick: Callback<MouseEvent> = {
        Callback::from(move |_| {
            lang_state.dispatch(!lang_state.is_eng);
            set_lang(!lang_state.is_eng);
        })
    };
    theme_provider::mount_on_theme_dom(switch_state);
    html! {
        <>
        <header>
        <nav class="nav-menu">
        <h2 class="logo">{"RestHub"} </h2>
        <input type="checkbox" id="active"/>
        <label for="active" class="menu-btn"><span></span></label>
        <label for="active" class="close"></label>
        <div class="wrapper">
        <ul>
        <li><Link<RootRoutes> to={RootRoutes::Root}>{ "Home" }</Link<RootRoutes>></li>
        <li><Link<RootRoutes> to={RootRoutes::About}>{ "About" }</Link<RootRoutes>></li>
        <li><Link<RootRoutes> to={RootRoutes::Search}>{ "Search" }</Link<RootRoutes>></li>
        </ul>
        <div class="toggle">
        <a>{"toggle dark mode"}</a>
        <input class="test" type="checkbox" id="switch" onclick={theme_onclik} checked={switch_state}/>
             <label class="tl" for="switch">
                 </label>
                 </div>

        <div class="toggle">
        <a>{"Chinese/English"}</a>
        <input class="test" type="checkbox" id="switch" onclick={lang_onclick} />
             <label class="tl" for="switch">
                 </label>
                 </div>
        </div>
        </nav>
        </header>
    </>
            }
}
