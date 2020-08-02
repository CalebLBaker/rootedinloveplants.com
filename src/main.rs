#[macro_use]
extern crate horrorshow;

fn main() {
    let content_file = std::env::args().nth(1).expect("Content file not specified");
    let content_read_result = std::fs::read_to_string(content_file.clone());
    let content_json = content_read_result.expect(format!("Failed to read {}", content_file).as_str());
    let parse_result = json::parse(content_json.as_str());
    let content = parse_result.expect(format!("Contents of {} are not valid JSON", content_file).as_str());

    let doc = format!("{}", html! {
        : horrorshow::helper::doctype::HTML;
        html {
            head {
                title : content["title"].as_str();
                link(rel="stylesheet", href="styles.css");
            }
            body {
                div(class="plant-list") {
                    @ for plant in content["plants"].members() {
                        div(class="plant") {
                            img(src=plant["picture"].as_str());
                            p { b : plant["name"].as_str() }
                            p : if let Some(price) = plant["price"].as_f64() {
                                format!("${:.2}", price)
                            }
                            else {
                                "".to_string()
                            }
                        }
                    }
                }
            }
        }
    });

    std::fs::write("index.html", doc.as_bytes()).expect("Failed to write out to file index.html");
}

