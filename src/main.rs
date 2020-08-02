#[macro_use]
extern crate horrorshow;

static mut NEXT_PLANT_ID : i64 = 0;

fn make_plant(plant : &json::JsonValue) -> Box<dyn horrorshow::RenderBox + '_> {
    let plant_id = unsafe {
        let plant_id = NEXT_PLANT_ID;
        NEXT_PLANT_ID += 1;
        plant_id
    };
    let id_str = plant_id.to_string();
    let image = plant["picture"].as_str();
    let name = plant["name"].as_str();
    let cost = if let Some(price) = plant["price"].as_f64() {
        format!("${:.2}", price)
    }
    else {
        "".to_string()
    };

    box_html! {
        a(class="plant", href=format!("#{}", id_str)) {
            img(src=image, class="thumbnail");
            p { b : name; }
            p : cost.clone();
        }
        div(style="display:none") {
            div(id=id_str, class="popup") {
                img(src=image, class="picture");
                h2 : name;
                h3 : cost;
                p(class="description") : plant["description"].as_str();
            }
        }
    }
}

fn main() {
    let content_file = std::env::args().nth(1).expect("Content file not specified");
    let content_read_result = std::fs::read_to_string(content_file.clone());
    let content_json = content_read_result.expect(format!("Failed to read {}", content_file).as_str());
    let parse_result = json::parse(content_json.as_str());
    let content = parse_result.expect(format!("Contents of {} are not valid JSON", content_file).as_str());

    let doc = owned_html! {
        : horrorshow::helper::doctype::HTML;
        html {
            head {
                title : content["title"].as_str();
                link(rel="stylesheet", href="fancybox/jquery.fancybox-1.3.4.css", type="text/css", media="screen");
                link(rel="stylesheet", href="styles.css", type="text/css");
            }
            body {
                script { :
                    // Initialize Facebook messenger API
                    "window.fbAsyncInit = function(){\
                        FB.init({\
                            appId: '720704568710810',\
                            autoLogAppEvents: true,\
                            xfbml: true,\
                            version: 'v0.1'\
                        });\
                    };\
                    (function(d, s, id){\
                        var js, fjs = d.getElementsByTagName(s)[0];\
                        if (d.getElementById(id)) return;\
                        js = d.createElement(s); js.id = id;\
                        js.src = 'https://connect.facebook.net/en_US/sdk/xfbml.customerchat.js';\
                        fjs.parentNode.insertBefore(js, fjs);\
                    }(document, 'script', 'facebook-jssdk'));"
                }
                script(async, defer, crossorigin="anonymous", src="https://connect.facebook.net/en_US/sdk.js");
                div(class="plant-list") {
                    @ for plant in content["plants"].members() {
                        : make_plant(plant);
                    }
                }
                div(style="display:none") {
                    div(id="data") : "blah blah blah";
                }
                script(type="text/javascript", src="https://ajax.googleapis.com/ajax/libs/jquery/1.4/jquery.min.js");
                script(type="text/javascript", src="fancybox/jquery.fancybox-1.3.4.pack.js");
                script(type="text/javascript", src="index.js");
            }
        }
    };

    std::fs::write("index.html", format!("{}", doc).as_bytes()).expect("Failed to write out to file index.html");
}

