use log::info;
use std::{env, fs};
use stylist::style;
use utils::theme_provider::Theme;
use view::{footer, nav};
use yew::prelude::*;
#[function_component(Content)]
pub fn content() -> Html {
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

    let t = std::include_str!("../../../post/test.md");
    log::info!("{}", t);
    html! {
        <>
            <nav::RhNav/>
                  <div class={&style.get_class_name().to_string()}>
                <article>
                <p>{t}</p>
                 </article>
                </div>

                //<footer::rh_footer/>
            </>
    }
}
