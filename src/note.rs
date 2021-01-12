extern crate uuid;
use uuid::Uuid;

#[derive(Debug)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub contents: String,
}

impl Note {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title: String::from(""),
            contents: String::from(""),
        }
    }
}
