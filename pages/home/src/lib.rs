use log::info;
use stylist::style;
use view::{footer, nav, post};
use yew::prelude::*;
#[function_component(Home)]
pub fn home() -> Html {
    let style = style!(
        r"
        .main_area {
        margin:50px;
        }
        .big_pic {
        position:relative;
        background-size: contain;
        background-repeat: no-repeat;
        display: flex;
        width: 80%;
        height: 80%;
        }
        .main_area {
            padding:  25px;
            margin: 0 auto;
            max-width: 1024px;
        }
      .kanban {
          box-shadow: 0 0.5rem 2rem rgb(0 0 0 / 12%);
        position:relative;
        overflow: hidden;
        border-radius: 0.5rem;
        display:flex;
        width:100%;
        margin-left:20px;
      } 
      .kanban_meta_data {
        overflow: hidden;
        width:15%; 
        flex-direction: column;
        justify-content: center;
        display: flex;
      }
      .kanban_meta_data * {
      padding-left: 15px
      }
      .kanban_content {
      padding-left:0px;
      }
      time {
      color:green;
      }
      .kanban:hover .big_pic {
        opacity: 0.5;
      }
      .grid-2_container{
      margin-left: 20px;
    display: flex;
    margin-left: 25px;
    margin: auto;
    max-width: 1024px;
    padding-left: 20px;
    flex-wrap: wrap;
      }

      .grid-2_article{
       box-shadow: 0 0.5rem 2rem rgb(0 0 0 / 12%);
       width: 50%;
       border-radius: 0.5rem;
       height: 50%;
       display: grid;
       flex:45%;
       margin-left:25px;
      }       
      .regular_pic {
        position:relative;
        background-size: contain;
        background-repeat: no-repeat;
        display: flex;
        width: 100%;
      }
      .grid-2_article_metadata {
      height: auto;
      }
      .grid-2_article_metadata * {
      padding-left: 15px;
      }
     .giu_content {
        height:0;
        opacity: 0;
        padding-left:0px;
    }
      .grid-2_article:hover .giu_content {
      height:75px;
      overflow: hidden;
      opacity: 1;
  transition: opacity 1.1s ease-out;
  
     }
      .grid-2_article:hover .regular_pic {
  transition: opacity 0.1s ease-out;
  opacity: 0.5;
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
                  <article>
                      <div class="kanban">
                      <img class="big_pic" src="https://live.staticflickr.com/65535/52573392542_eeb51ca196_4k.jpg"/>
                      <div class="kanban_meta_data">
                      <time>{"March 05, 2019"}</time>
                      <h2>{"Emoji Support"}</h2>
                      <div>
                          <p class="kanban_content">
                          {"Emoji can be enabled in a Hugo project in a number of ways."}
                          </p>
                      </div>
                      </div>
                      </div>
                  </article>
                  </div>
                  <div class="grid-2_container">
                    <article class="grid-2_article">
                      <img class="regular_pic" src="https://live.staticflickr.com/65535/52573392542_eeb51ca196_4k.jpg"/>
                      <div class="grid-2_article_metadata">
                      <time>{"March 05, 2019"}</time>
                      <h2>{"Emoji Support"}</h2>
                      <div>
                          <p class="giu_content">
                          {"Emoji can be enabled in a Hugo project in a number of ways."}
                          </p>
                      </div>
                      </div>
                    </article>
                    <article class="grid-2_article">
                      <img class="regular_pic" src="https://live.staticflickr.com/65535/52573392542_eeb51ca196_4k.jpg"/>
                      <div class="grid-2_article_metadata">
                      <time>{"March 05, 2019"}</time>
                      <h2>{"Emoji Support"}</h2>
                      <div>
                          <p class="giu_content">
                          {"Emoji can be enabled in a Hugo project in a number of ways."}
                          </p>
                      </div>
                      </div>
                    </article>
                    <post::PostComponent ..post.clone() />
                  </div>
                </div>
                <footer::rh_footer/>
            </>
    }
}
