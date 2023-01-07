use log::info;
use pulldown_cmark::{html, Options, Parser};
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
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(t, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    log::info!("{}", html_output);
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
