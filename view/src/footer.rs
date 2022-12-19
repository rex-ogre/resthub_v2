use stylist::style;
use yew::prelude::*;

#[function_component]
pub fn rh_footer() -> Html {
    let styles = style!(
        r"
         .left {
  position: absolute;
  bottom: 25px;
  display:flex;
  left: 55px;
  /*left: 20px; */
  color: #ABB2B9;
         }
         .right {
  position: absolute;
  bottom: 25px;
  display:flex;
  z-index: 2;
  right: 0px;
  /*left: 20px; */
  color: #E8D0CB;
  }
        "
    )
    .unwrap();
    html! {
        <>
        <footer>
            <div class={&styles.get_class_name().to_string()} >
            <p class={"left"}> {"© 2022 RestHubs"}</p>
            <p class={"right"} > {"designed by Rex123"}</p>
            </div>
        </footer>
        </>
    }
}
