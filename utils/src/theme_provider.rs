use log::info;
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
impl Theme {
    pub fn from_storage(local_storage: String) -> Theme {
        if local_storage == "dark" {
            Theme { dark_theme: true }
        } else {
            Theme { dark_theme: false }
        }
    }
}
const THEME_KEY: &'static str = "THEME";
pub fn get_theme_from_storage() -> Theme {
    let local_storage = window()
        .unwrap()
        .local_storage()
        .unwrap()
        .expect("unwrap local_storage failed");

    info!("local_storage.get_item....");
    match local_storage.get_item(THEME_KEY).unwrap() {
        Some(theme) => {
            info!("get from local_storage theme is {}", theme.to_string());
            Theme::from_storage(theme.to_string())
        }
        None => match use_prefered_dark() {
            true => {
                info!("doesn't get from storage, default is dark");
                set_theme(true);
                Theme::from_storage("dark".to_string())
            }
            false => {
                info!("doesn't get from storage, default is light");
                set_theme(false);
                Theme::from_storage("light".to_string())
            }
        },
    }
}
pub fn set_theme(theme: bool) {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    info!("set_theme {}", theme);
    match theme {
        true => local_storage.set_item(THEME_KEY, "dark").unwrap(),
        false => local_storage.set_item(THEME_KEY, "light").unwrap(),
    };
}

pub fn mount_on_theme_dom(is_dark: bool) {
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
    log::info!("render theme here,true means dark  {} ", is_dark);
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
    info!("use_prefered_dark{}", is_perfered_dark);
    is_perfered_dark
}
