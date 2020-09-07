#[macro_use]
extern crate horrorshow;

use serde::Deserialize;

static mut NEXT_PLANT_ID : i64 = 0;

#[derive(Deserialize)]
struct Plant {
    picture: String,
    name: String,
    price: f64,
    description: String,
}

#[derive(Deserialize)]
struct Button {
    icon: String,
    text: String,
}

#[derive(Deserialize)]
struct Content {
    title: String,
    icon: String,
    banner: String,
    email_button: Button,
    plants: Vec<Plant>,
    success_toast: String,
    error_toast: String,
    address_label: String,
    subject_label: String,
    body_label: String
}

fn make_plant<'a>(plant : &'a Plant, email_button : &'a Button) -> Box<dyn horrorshow::RenderBox + 'a> {
    let plant_id = unsafe {
        let plant_id = NEXT_PLANT_ID;
        NEXT_PLANT_ID += 1;
        plant_id
    };
    let id_str = plant_id.to_string();
    let cost = format!("${:.2}", plant.price);
    let image = plant.picture.as_str();
    let name = plant.name.as_str();

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
                p(class="description") : plant.description.as_str();
                div(onclick=format!("displayEmailForm('{}');", name), class="emailButton"){
                    img(src=email_button.icon.as_str(), class="emailIcon");
                    p(class="emailButtonText") : email_button.text.as_str();
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let content_file = std::fs::File::open(args.nth(1).ok_or("Content file not specified")?)?;
    let stylesheet = std::fs::read_to_string(args.next().ok_or("Stylesheet not specified")?)?;
    let minified_css = minifier::css::minify(stylesheet.as_str())?;
    let script = std::fs::read_to_string(args.next().ok_or("Script file not specified")?)?;
    let content : Content = serde_json::from_reader(std::io::BufReader::new(content_file))?;

    let doc = owned_html! {
        : horrorshow::helper::doctype::HTML;
        html {
            head {
                title : content.title.as_str();
                link(rel="stylesheet", href="fancybox/jquery.fancybox-1.3.4.css", type="text/css", media="screen");
                link(rel="icon", href=content.icon.as_str());
                style : horrorshow::Raw(minified_css.as_str());
            }
            body {
                header {
                    img(id="banner", src=content.banner.as_str());
                }
                div(id="plant-list") {
                    @ for plant in content.plants.iter() {
                        : make_plant(plant, &content.email_button);
                    }
                }
                div(style="display:none") {
                    a(id="emailLink", href="#emailForm");
                    div(id="emailForm") {
                        label: content.address_label.as_str();
                        input(id="emailAddress", type="email");
                        label: content.subject_label.as_str();
                        input(id="subject", type="text");
                        label: content.body_label.as_str();
                        textarea(id="body", rows="16", cols="120");
                        button(onclick="sendEmail();") : "Send";
                    }
                }
                div(id="successToast", class="toast") : content.success_toast.as_str();
                div(id="errorToast", class="toast") : content.error_toast.as_str();
                script(type="text/javascript", src="https://ajax.googleapis.com/ajax/libs/jquery/1.4/jquery.min.js");
                script(type="text/javascript", src="fancybox/jquery.fancybox-1.3.4.pack.js");

                // In production, use inline minified javascript
                // When debugging, switch which line is commented out to just include the external
                // js file
                script(type="text/javascript") : horrorshow::Raw(minifier::js::minify(script.as_str()));
                // script(type="text/javascript", src="index.js");
            }
        }
    };
    std::fs::write("index.html", format!("{}", doc).as_bytes())?;
    Ok(())
}

