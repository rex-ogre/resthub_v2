use utils::*;
use yew::prelude::*;
#[function_component]
pub fn Nav() -> Html {
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
        <li><a href="#">{"Home"}</a></li>
        <li><a href="#">{"About"}</a></li>
        <li><a href="#">{"Services"}</a></li>
        <li><a href="#">{"Gallery"}</a></li>
        <li><a href="#">{"Feedback"}</a></li>
        </ul>
        </div>
        /*
        <div class="content">
        <div class="title">{"
        Fullscreen Overlay Navigation Bar"}</div>
        <p>
        {"(Hamburger Menu-2)"} </p>
        </div>
        */
        </nav>
        </header>
    </>
            }
}
