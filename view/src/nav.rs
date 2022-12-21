use log::info;
use std::rc::Rc;
use utils::theme_provider::{self, Theme};
use yew::prelude::*;
#[function_component]
pub fn rhNav() -> Html {
    let dark_state = use_context::<UseReducerHandle<Theme>>().unwrap();
    let switch_state: bool = dark_state.dark_theme;
    let onclick: Callback<MouseEvent> = {
        Callback::from(move |_| {
            dark_state.dispatch(!dark_state.dark_theme);
            theme_provider::set_theme(!dark_state.dark_theme);
            // theme_provider::mount_on_dom(dark_state.dark_theme);
        })
    };

    theme_provider::mount_on_dom(switch_state);
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
        <li><p>{"Home"}</p></li>
        <li><p>{"About"}</p></li>
        <li><p>{"Services"}</p></li>
        </ul>
        <div class="toggle">
        <p>{"Toggle Dark Mode"}</p>
        <input class="test" type="checkbox" id="switch" onclick={onclick} checked={switch_state}/>
             <label class="tl" for="switch">
                 </label>
                 </div>
        </div>
        </nav>
        </header>
    </>
            }
}
