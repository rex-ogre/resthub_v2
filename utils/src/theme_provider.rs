use wasm_bindgen::JsValue;
use web_sys::window;
use yew::prelude::*;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Theme {
    pub dark_theme: bool,
}
impl Reducible for Theme {
    type Action = bool;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        Theme { dark_theme: action }.into()
    }
}

pub fn mount_on_dom(is_dark: bool) {
    let window = window().expect("should have window");
    let document = window.document().expect("should have document");
    let arr = js_sys::Array::new_with_length(1);
    arr.set(0, JsValue::from_str("dark"));
    if is_dark {
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
pub fn use_prefered_dark() -> bool {
    let window = web_sys::window().expect("no global `window` exists");
    let mut is_perfered_dark = false;
    match window.match_media("(prefers-color-scheme: dark)") {
        Ok(option_media_query_list) => match option_media_query_list {
            Some(media_query_list) => {
                is_perfered_dark = media_query_list.matches();
            }
            None => {}
        },
        Err(_) => {}
    };
    is_perfered_dark
}
