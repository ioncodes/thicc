use rocket::response::{Redirect, NamedFile};
use rocket::Config;
use rocket::config::Environment;
use rocket::response::content::Html;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use askama::Template;
use crate::db::Db;
use crate::constants::{PROTOCOL, HOSTNAME, PORT};
use crate::create_template::CreateTemplate;
use crate::paste_template::PasteTemplate;
use crate::paste::Paste;
use std::path::Path;

pub struct Server { }

#[get("/create")]
fn create_html() -> Html<String> {
    let template = CreateTemplate { protocol: PROTOCOL, host: HOSTNAME, port: &PORT.to_string()[..] };
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
    let decoded = String::from(std::str::from_utf8(&base64::decode(&paste.code).unwrap()).unwrap());
    let code = &urldecode::decode(decoded)[..];
    let template = PasteTemplate { code , language };
    Html(template.render().unwrap())
}

#[get("/<id>/raw")]
fn paste_raw(id: String) -> String {
    let paste = Db::get_paste(id);
    let language: &str = &paste.language;
    let decoded = String::from(std::str::from_utf8(&base64::decode(&paste.code).unwrap()).unwrap());
    let code = urldecode::decode(decoded);
    code
}

#[get("/favicon.ico")]
fn icon() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/icons/favicon.ico")).ok()
}

impl Server {
    pub fn new() -> Server { Server { } }

    pub fn start(&self) {
        Db::initialize();

        let config = Config::build(Environment::Staging)
            .address("127.0.0.1")
            .port(7000)
            .finalize()
            .unwrap();
        rocket::custom(config)
            .mount("/js", StaticFiles::from("static/js"))
            .mount("/css", StaticFiles::from("static/css"))
            .mount("/", routes![create_html, create_paste, paste, paste_raw, icon])
            .launch();
    }
}