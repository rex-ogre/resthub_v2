pub use crate::post::PostView;
use stylist::style;
use yew::prelude::*;
#[function_component]
pub fn KanbanComponent(props: &PostView) -> Html {
    let style = style!(
        r"
        .big_pic {
        position:relative;
        background-size: contain;
        background-repeat: no-repeat;
        display: flex;
        width: 80%;
        height: 80%;
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
      padding-left:0px
      }
      .kanban:hover .big_pic {
        opacity: 0.5;
      }
@media screen and (max-width: 600px){
    .kanban {
    margin-left:auto;
    }
    .big_pic {
    width:60%;
    height:auto;
    }
    .kanban_meta_data {
    width:auto;
    }
}
        "
    )
    .unwrap();
    html!(
    <>
              <article class={style.get_class_name().to_string()}>
                  <div class="kanban">
                  <img class="big_pic" src={props.img.to_owned()}/>
                  <div class="kanban_meta_data">
                  <time>{props.time.to_owned()}</time>
                  <h2>{props.title.to_owned()}</h2>
                  <div>
                      <p class="kanban_content">
                      {props.content.to_owned()}
                      </p>
                  </div>
                  </div>
                  </div>
              </article>
    </>
    )
}
