use sqlite::{Connection, Value};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use crate::paste::Paste;

pub struct Db { }

impl Db {
    pub fn initialize() {
        let connection = sqlite::open("./pastes.db").unwrap();
        let _ = connection.execute("CREATE TABLE IF NOT EXISTS pastes (id TEXT UNIQUE, code TEXT, language TEXT)");
    }

    pub fn create_paste(paste: Paste) -> String {
        let id: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(50)
            .collect();

        let connection = sqlite::open("./pastes.db").unwrap();

        let mut cursor = connection
            .prepare("INSERT INTO pastes VALUES (?, ?, ?)")
            .unwrap()
            .cursor();
        cursor.bind(&[Value::String(id.clone()), Value::String(paste.code), Value::String(paste.language)]).unwrap();

        let _ = cursor.next();

        id
    }

    pub fn get_paste(id: String) -> Paste {
        let connection = sqlite::open("./pastes.db").unwrap();

        let mut cursor = connection
            .prepare("SELECT code, language FROM pastes WHERE id = ?")
            .unwrap()
            .cursor();
        cursor.bind(&[Value::String(id.clone())]).unwrap();

        let row = cursor.next().unwrap().unwrap();

        let code = row[0].as_string().unwrap();
        let language = row[1].as_string().unwrap();

        Paste {
            code: String::from(code),
            language: String::from(language)
        }
    }
}