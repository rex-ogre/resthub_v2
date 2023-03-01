use log::info;
use wasm_bindgen::JsValue;
use web_sys::window;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LangRrovider {
    pub is_eng: bool,
}

impl Reducible for LangRrovider {
    type Action = bool;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        LangRrovider { is_eng: action }.into()
    }
}

impl LangRrovider {
    pub fn from_storage(local_storage: String) -> LangRrovider {
        if local_storage == "en" {
            LangRrovider { is_eng: true }
        } else {
            LangRrovider { is_eng: false }
        }
    }
}
const LANG_KEY: &'static str = "LANG";
pub fn get_lang_from_storage() -> LangRrovider {
    let local_storage = window()
        .unwrap()
        .local_storage()
        .unwrap()
        .expect("unwrap local_storage failed");

    info!("local_storage.get_item....");
    match local_storage.get_item(LANG_KEY).unwrap() {
        Some(lang) => {
            info!("get from lang theme is {}", lang.to_string());
            LangRrovider::from_storage(lang.to_string())
        }
        None => {
            info!("no default language");
            LangRrovider { is_eng: true }
        }
    }
}
pub fn set_lang(lang: bool) {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    info!("set_lang {}", lang);
    match lang {
        true => local_storage.set_item(LANG_KEY, "dark").unwrap(),
        false => local_storage.set_item(LANG_KEY, "light").unwrap(),
    };
}
pub fn mount_on_lang_dom(is_eng: bool) {
    let window = window().expect("should have window");
    let document = window.document().expect("should have document");
    let arr = js_sys::Array::new_with_length(1);
    arr.set(0, JsValue::from_str("eng"));
    if is_eng {
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
    log::info!("change lang{}, true is eng,false is cn", is_eng);
}
