use askama::Template;

#[derive(Template)]
#[template(path = "create.html")]
pub struct CreateTemplate<'a> {
    pub protocol: &'a str,
    pub host: &'a str,
    pub port: &'a str
}