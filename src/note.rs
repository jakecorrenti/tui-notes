pub struct Note {
    pub title: String,
    pub contents: String,
    pub date_created: String, 
}
 
impl Note {
    pub fn new() -> Self {
        Self {
            title: String::from(""),
            contents: String::from(""),
            date_create: String::from(""),
        }
    }
}
