use wasm_bindgen::JsValue;
use web_sys::window;
use yew::prelude::*;

pub fn mount_on_dom(is_dark: bool) {
    let window = window().expect("should have window");
    let document = window.document().expect("should have document");
    let arr = js_sys::Array::new_with_length(1);
    arr.set(0, JsValue::from_str("dark"));
    if !is_dark {
        document
            .body()
            .expect("no header")
            .class_list()
            .add(&arr)
            .expect("add dark failed");
    } else {
        document
            .body()
            .expect("no header")
            .class_list()
            .remove(&arr)
            .expect("remove dark failed");
    }
    log::info!("element_info is here,{} ", is_dark);
}
