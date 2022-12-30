use log::info;
use stylist::style;
use view::{footer, kanban, nav, post};
use yew::prelude::*;
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
    let post = post::Post {
        img: String::from("https://live.staticflickr.com/65535/52573392542_eeb51ca196_4k.jpg"),
        time: String::from("March 05, 2019"),
        title: String::from("Emoji Support"),
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
