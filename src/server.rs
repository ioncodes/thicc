use rocket::response::Redirect;
use rocket::Config;
use rocket::config::Environment;
use rocket::response::content::Html;
use rocket_contrib::json::Json;
use askama::Template;
use crate::db::Db;
use crate::constants::{PROTOCOL, HOSTNAME, PORT};
use crate::create_template::CreateTemplate;
use crate::paste_template::PasteTemplate;
use crate::paste::Paste;

pub struct Server { }

#[get("/create")]
fn create_html() -> Html<String> {
    let template = CreateTemplate { name: "Layle" };
    Html(template.render().unwrap())
}

#[post("/create", format = "application/json", data = "<paste>")]
fn create_paste(paste: Json<Paste>) -> String {
    let id = Db::create_paste(paste.0);
    id
}

#[get("/<id>")]
fn paste(id: String) -> Html<String> {
    let paste = Db::get_paste(id);
    let language: &str = &paste.language;
    let decoded = &base64::decode(&paste.code).unwrap();
    let code: &str = std::str::from_utf8(decoded).unwrap();
    let template = PasteTemplate { code , language };
    Html(template.render().unwrap())
}

impl Server {
    pub fn new() -> Server { Server { } }

    pub fn start(&self) {
        Db::initialize();

        let config = Config::build(Environment::Staging)
            .address("0.0.0.0")
            .port(PORT)
            .finalize()
            .unwrap();
        rocket::custom(config).mount("/", routes![create_html, create_paste, paste]).launch();
    }
}