use gloo_net::http::Request;
use log::info;
use std::collections::HashMap;
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
    let videos = use_state(|| String::from(""));
    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: String = Request::new("http://127.0.0.1:3002/json")
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();

                    videos.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }
    let post = post::Post {
        img: String::from("https://live.staticflickr.com/65535/52573392542_eeb51ca196_4k.jpg"),
        time: String::from("March 05, 2019"),
        title: videos.to_string(),
        content: String::from("Emoji can be enabled in a Hugo project in a number of ways."),
    };
    html! {
        <>
            <nav::RhNav/>
                  <div class={&style.get_class_name().to_string()}>
                <div class="main_area">
                <kanban::KanbanComponent ..post.clone() />
                  </div>
                  <div class="grid-2_container">
                    <post::PostComponent ..post.clone() />
                    <post::PostComponent ..post.clone() />
                    <post::PostComponent ..post.clone() />
                    <post::PostComponent ..post.clone() />
                  </div>
                </div>
                <footer::rh_footer/>
            </>
    }
}
