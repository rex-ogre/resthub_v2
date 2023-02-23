use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    filename: String,
    date: String,
    title: String,
    info: String,
}
impl Post {
    pub fn new(content: &str, filename: &str) -> Self {
        let date_result = content.find("<h6>");
        let _date: String = match date_result {
            Some(_) => content
                [content.find("<h6>").expect("no date data") + 4..content.find("</h6>").unwrap()]
                .to_string(),

            None => String::from("no time data"),
        };

        let _title: String = match date_result {
            Some(_) => {
                content[content.find(">").unwrap() + 1..content.find("</h1>").unwrap()].to_string()
            }
            None => String::from("no title"),
        };

        let _content_index = content.find("<hr />");
        let _content_slice = match _content_index {
            Some(first) => {
                let _content_second_index =
                    content[first + 7..content.len()].find("<hr />").to_owned();

                match _content_second_index {
                    Some(sec) => {
                        let _content_slice = content[first..sec + first]
                            .replace("<p>", "")
                            .replace("<hr />", "")
                            .replace("\n", "")
                            .replace("</p>", "")
                            .replace("<em>", "")
                            .replace("</em>", "")
                            .replace("<strong>", "")
                            .replace("</strong>", "")
                            .replace("<code>", "")
                            .replace("</code>", "")
                            .replace("<pre>", "")
                            .replace("</pre>", "")
                            .replace("<blockquote>", "")
                            .replace("</blockquote>", "")
                            .replace("<h1>", "")
                            .replace("</h1>", "")
                            .replace("<h2>", "")
                            .replace("</h2>", "")
                            .replace("<h3>", "")
                            .replace("</h3>", "")
                            .replace("<h4>", "")
                            .replace("</h4>", "")
                            .replace("<h5>", "")
                            .replace("</h5>", "")
                            .replace("<h6>", "")
                            .replace("</h6>", "");
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

        Post {
            filename: filename.to_string(),
            date: _date,
            title: _title,
            info: _content_slice,
        }
    }
}
