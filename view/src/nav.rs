use log::info;
use utils::theme_provider;
use yew::prelude::*;
#[function_component]
pub fn rhNav() -> Html {
    let dark_state = use_state(|| false);
    let onclick: Callback<MouseEvent> = {
        Callback::from(move |_| {
            let dark_state = dark_state.clone();
            theme_provider::mount_on_dom(*dark_state);
            info!("Hello ");
            dark_state.set(!*dark_state);
        })
    };
    html! {
            <>
                <head>

        <link href="https://fonts.googleapis.com/css2?family=Oswald&display=swap" rel="stylesheet"/>
        <link href="https://fonts.googleapis.com/css2?family=Lato&display=swap" rel="stylesheet"/>
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
        <link href="https://fonts.googleapis.com/css2?family=Pacifico&display=swap" rel="stylesheet"/>
          </head>

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
        <input class="test" type="checkbox" id="switch" onclick={onclick} />
             <label class="tl" for="switch">
                 </label>
                 </div>
        </div>
        </nav>
        </header>
    </>
            }
}
