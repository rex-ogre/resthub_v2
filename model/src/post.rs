use serde::{Deserialize, Serialize};
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub filename: String,
    pub date: String,
    pub title: String,
    pub info: String,
    pub image: String,
    pub content: String,
}
impl Post {
    pub fn new(content: &str, filename: &str) -> Self {
        let date_result = content.find("######");
        let _date: String = match date_result {
            Some(i) => {
                let second_index = content[i..content.len()].find("\n").unwrap() + i;
                content[i + "######".len()..second_index].to_string()
            }

            None => String::from("no time data"),
        };
        let title_result = content.find("#");

        let _title: String = match title_result {
            Some(i) => {
                let second_index = content[i..content.len()].find("\n").unwrap();
                String::from(&content[i + 1..second_index])
            }
            None => String::from("no title"),
        };

        let _content_index = content.find("---");
        let _content_slice = match _content_index {
            Some(first) => {
                let _content_second_index =
                    content[first + 3..content.len()].find("---").to_owned();

                match _content_second_index {
                    Some(sec) => {
                        let _content_slice = &content[first..sec + first]
                            .replace("\n", "")
                            .replace("---", "")
                            .replace("#", "");
                        if _content_slice.len() > 60 {
                            String::from(&_content_slice[0..60])
                        } else {
                            String::from(_content_slice)
                        }
                    }
                    None => String::from("no content"),
                }
            }
            None => String::from("no content"),
        };
        let _image_index = content.find("![image](");
        let _image_slice = match _image_index {
            Some(i) => {
                let second_index = content[i..content.len()].find(")").unwrap();
                content[i + "![image](".len()..i + second_index].to_string()
            }
            None => String::from("nil"),
        };
        Post {
            filename: filename.to_string(),
            date: _date,
            title: _title,
            info: _content_slice,
            content: content.to_string(),
            image: _image_slice,
        }
    }
}
