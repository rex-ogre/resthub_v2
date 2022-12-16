use js_sys::Array;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::{window, Headers};
use yew::prelude::*;
pub fn mount_on_dom() {
    let window = window().expect("should have window");
    let document = window.document().expect("should have document");
    let element = document.document_element().expect("should have element");
    let class_list = element.class_list();
    let arr = js_sys::Array::new_with_length(1);
    arr.set(0, JsValue::from_str("dark"));
    class_list.add(&arr).expect("should add dark class success");

    document
        .body()
        .expect("no header")
        .class_list()
        .add(&arr)
        .expect("add dark failed");
    log::info!("element_info is here, {:?}", class_list);
}
#[function_component(ToggleTheme)]
pub fn toggle_theme() -> Html {
    let onclick: Callback<MouseEvent> = {
        Callback::from(move |_| {
            mount_on_dom();
        })
    };
    html! {
      <button  type="button" value={"1244 "} onclick={onclick}>{"切換主題"}</button>
    }
}
