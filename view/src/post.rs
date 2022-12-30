use stylist::style;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Post {
    pub img: String,
    pub time: String,
    pub title: String,
    pub content: String,
}
#[function_component]
pub fn PostComponent(props: &Post) -> Html {
    let styles = style!(
        r"
    box-shadow: 0 0.5rem 2rem rgb(0 0 0 / 12%);
    width: 47.5%;
    border-radius: 0.5rem;
    height: 50%;
    display: grid;
    margin-left: 2.5%;
    margin-top: 2%;

      .regular_pic {
        position:relative;
        background-size: contain;
        background-repeat: no-repeat;
        display: flex;
        width: 100%;
        border-radius: 0.5rem;
      }
      .grid-2_article_metadata {
      height: auto;
      margin-bottom:20px;
      }
      .grid-2_article_metadata * {
      padding-left: 15px;
      }
     .giu_content {
        height:0;
        opacity: 0;
        padding-left:0px;
    }
      :hover .giu_content {
      height:75px;
      overflow: hidden;
      opacity: 1;
  transition: opacity 1.1s ease-out;
  
     }
      :hover .regular_pic {
  transition: opacity 0.1s ease-out;
  opacity: 0.5;
}

@media screen and (max-width: 600px){
    width:100%;
    margin-right:2.5%;
    margin-left:auto;
    
}
        "
    )
    .unwrap();

    html! {
    <>
                    <article class={classes!("grid-2_article",{&styles.get_class_name().to_string()})}>
                      <img class="regular_pic" src={props.img.to_owned()}/>
                      <div class="grid-2_article_metadata">
                      <time>{props.time.to_owned()}</time>
                      <h2>{props.title.to_owned()}</h2>
                      <div>
                          <p class="giu_content">
                          {props.content.to_owned()}
                          </p>
                      </div>
                      </div>
                    </article>
                    </>
    }
}
