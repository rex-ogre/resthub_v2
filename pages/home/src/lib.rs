use log::info;
use stylist::style;
use view::{footer, nav};
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
        position:relative;
        overflow: hidden;
        border-radius: 0.5rem;
        display:flex;
        width:100%;
        grid-gap: 15px;
        margin-left:20px;
      } 
      .kanban_meta_data {
        overflow: hidden;
        width:15%; 
        justify-content: center;
        flex-direction: column;
        display: flex;
        grid-gap: 2px;
      }
      time {
      color:green;
      }
      .grid-2_container{
        justify-content: center;
        flex-direction: column;
          display: grid;
      }

      .grid-2_article{
          flex: 1;
      }       
      .regular_pic {
        position:relative;
        background-size: contain;
        background-repeat: no-repeat;
        display: flex;
        width: 35%;
        height: 35%;
      }
    "
    )
    .unwrap();
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
                      </div>
                    </article>
                    <article class="grid-2_article">
                      <img class="regular_pic" src="https://live.staticflickr.com/65535/52573392542_eeb51ca196_4k.jpg"/>
                      <div class="grid-2_article_metadata">
                      </div>
                    </article>
                  </div>
                </div>
                <footer::rh_footer/>
            </>
    }
}
