use log::info;
use stylist::style;
use utils::theme_provider::Theme;
use view::{footer, nav};
use yew::prelude::*;
#[function_component(Home)]
pub fn home() -> Html {
    let style = style!(
        r"
        .title {
            position: relative;
            left: 25%;
            font-size: 200%;
            font-weight: 400;
        }
        .date {
            position: relative;
            left: 25%;
            color: green;
            font-size: 120%;
            font-weight: 400;
        }
        .content {
        margin: 2em ;
        transform: translate(-25%, 0%);
        position: relative;
        width:70%;
        left: 35%;
        display:flex;
        }
    "
    )
    .unwrap();
    let theme = use_context::<UseReducerHandle<Theme>>().expect("no ctx found");
    html! {
        <>
            <nav::RhNav/>
                  <div class={&style.get_class_name().to_string()}>
                <article>
            <p class="date">
             {"09. April 2014" }
            </p>
            <h1 class="title">
            {"About Rex"}{theme.dark_theme.to_string()}
            </h1>
                <p class="content">{"Hugo is the <strong>world’s fastest framework for building websites</strong>. It is written in Go."}</p>
                <p class="content">{"It makes use of a variety of open source projects including:"}</p>

                <p class="content">{"ety of open source projects including:"}</p>
                <p class="content">{"open source projects including:"}</p>
                 </article>
                </div>
                <footer::rh_footer/>
            </>
    }
}
