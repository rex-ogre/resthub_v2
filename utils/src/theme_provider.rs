use std::rc::Rc;
use wasm_bindgen::prelude::{js_sys, JsValue};
use web_sys::window;
use yew::prelude::*;
pub fn mount_on_dom() {
    let window = window().expect("should have window");
    let document = window.document().expect("should have document");
    let element = document.document_element().expect("should have element");
    let element_info = element.to_string();
    log::info!("{element_info}");
}
fn set_class(is_dark: bool) {
    // 下面的操作展示了獲取html元素並添加`dark`class的過程
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let element = document
        .document_element()
        .expect("should hav a element on document");
    // js_sys提供了數組數據結構用於添加到Element結構體中
    let arr = js_sys::Array::new_with_length(1);
    arr.set(0, JsValue::from_str("dark"));
    let class_list = element.class_list();
    if is_dark {
        class_list.add(&arr).expect("should add dark class success");
    } else {
        class_list
            .remove(&arr)
            .expect("should remove dark class success");
    }
}
#[function_component(ToggleTheme)]
pub fn toggle_theme() -> Html {
    let onclick = {
        mount_on_dom();
    };
    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}
