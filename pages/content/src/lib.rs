use log::info;
use pulldown_cmark::{html, Options, Parser};
use std::{env, fs};
use stylist::style;
use utils::theme_provider::Theme;
use view::{footer, nav};
use yew::prelude::*;
use web_sys::Element;

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
                <Post/>
                 </article>
                </div>

                //<footer::rh_footer/>
            </>
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub inner_html: String,
}
#[function_component(Post)]
pub fn post() -> Html {
    let t = std::include_str!("../../../post/test.md");
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(t, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    let node_ref = NodeRef::default();
    {
        let inner_html = html_output.clone();
        let node_ref = node_ref.clone();
        
        use_effect(move || {
            let el = node_ref.cast::<Element>().unwrap();
            el.set_inner_html(inner_html.as_str());
            || {}
        });
    }
    html! {
      <>
        <div>
          <h1>{"Yew Tailwindcss"}</h1>
        </div>
        <div ref={node_ref.clone()} />
      </>
    }
}
