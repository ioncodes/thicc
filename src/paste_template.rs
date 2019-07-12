use askama::Template;

#[derive(Template)]
#[template(path = "paste.html")]
pub struct PasteTemplate<'a> {
    pub language: &'a str,
    pub code: &'a str,
}