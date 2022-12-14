use stylist::css;
use stylist::style;
use view::nav;
use yew::prelude::*;
#[function_component(About)]
pub fn about() -> Html {
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
        .imgfornow {
        margin: 2em ;
        height: 800px;
        width: 75%;
        padding: 0;
        background: pink;
        position: relative;
        left: 50%;
        transform: translate(-50%, 0%);
        }
        .content {
        margin: 2em ;
        transform: translate(-25%, 0%);
        position: relative;
        width:70%;
        left: 35%;
        display:flex;
        }
        ul.content {
        margin: 2em ;
        transform: translate(-25%, 0%);
        position: relative;
        left: 15%;
        background: yellow;
        }
    "
    )
    .unwrap();
    html! {
        <>
            <nav::Nav/>
                <body>
                  <div class={&style.get_class_name().to_string()}>
                <article>
            <p class="date">
             {"09. April 2014" }
            </p>
            <h1 class="title">
            {"About Rex"}
            </h1>
                         <div class="imgfornow">
                </div>
                <p class="content">{"Hugo is the <strong>world’s fastest framework for building websites</strong>. It is written in Go."}</p>
                <p class="content">{"It makes use of a variety of open source projects including:"}</p>

                <p class="content">{"ety of open source projects including:"}</p>
                <p class="content">{"open source projects including:"}</p>
                 </article>
                </div>
                </body>
            </>
    }
}
