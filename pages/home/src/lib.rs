use gloo_net::http::Request;
use log::info;
use model::post::Post;
use serde_json::json;
use std::{collections::HashMap, iter::Map};
use stylist::style;
use view::{footer, kanban, nav, post};
use wasm_bindgen_futures::{future_to_promise, spawn_local};
use yew::{prelude::*, suspense::use_future};

#[function_component(Home)]
pub fn home() -> Html {
    let style = style!(
        r"
        .main_area {
        margin:50px;
        }
      time {
      color:green;
      }
        .main_area {
            padding:  25px;
            margin: 0 auto;
            max-width: 1024px;
        }
      .grid-2_container{
      display: flex;
    margin: auto;
    max-width: 1024px;
    padding-left: 20px;
    flex-wrap: wrap;
      }

    "
    )
    .unwrap();
    let data = use_state(|| Vec::<Post>::new());
    {
        let data = data.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_data: Vec<Post> = Request::new("http://127.0.0.1:3002/json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    log::info!("{:?}", &fetched_data);
                    let p: Vec<Post> =
                        serde_json::from_value(json!(fetched_data)).expect("parse failed");
                    log::info!("{:?}", p);
                    data.set(fetched_data);
                });
                || ()
            },
            (),
        );
    }
    let post = post::PostView {
        img: String::from("https://pgw.udn.com.tw/gw/photo.php?u=https://uc.udn.com.tw/photo/2023/02/24/realtime/20285625.jpg&x=0&y=0&sw=0&sh=0&sl=W&fw=800&exp=3600&w=400&nt=1"),
        time: String::from("March 05, 2019"),
        title: String::from("title"),
        content: String::from("Emoji can be enabled in a Hugo project in a number of ways."),
    };

    html! {
        <>
            <nav::RhNav/>
                  <div class={&style.get_class_name().to_string()}>
                <div class="main_area">
                   {for data.first().map(|comment| {
                        let c = comment.clone();
                        html! {
                            <kanban::KanbanComponent ..
                                post::PostView {
                                    img: c.image,
                                    time: c.date,
                                    title: c.title,
                                    content: c.info,
                                }
                                 />
                        }
                    })}

                  </div>
                  <div class="grid-2_container">
                   {
                       for data.iter().map(|comment| {
                           if comment == data.first().unwrap() {
                               return html!{};
                           }
                        let c = comment.clone();
                        html! {
                            <post::PostComponent ..
                                post::PostView {
                                    img: c.image,
                                    time: c.date,
                                    title: c.title,
                                    content: c.info,
                                }
                                 />
                        }
                    })}
                  </div>
                </div>
                <footer::rh_footer/>
            </>
    }
}
