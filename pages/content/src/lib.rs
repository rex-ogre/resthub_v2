use log::info;
use stylist::style;
use utils::theme_provider::Theme;
use view::{footer, nav};
use yew::prelude::*;
#[function_component(Search)]
pub fn search() -> Html {
    let style = style!(
        r"
        .title {
            position: relative;
            left: 25%;
            font-size: 200%;
            font-weight: 400;
        }
        .search__input {
            position: relative;
            left: 25%;
            width: 50%;
            border-radius: 999px;
            padding: 1.2rem 1.2rem 1.2rem 4.8rem;
        }
    "
    )
    .unwrap();
    html! {
        <>
            <nav::RhNav/>
                  <div class={&style.get_class_name().to_string()}>
                <article>
                <input class="search__input" id="searchInput" placeholder="Search contents..."/>
                 </article>
                </div>

                //<footer::rh_footer/>
            </>
    }
}
