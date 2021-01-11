extern crate rusqlite;
use rusqlite::{Connection, Result, NO_PARAMS};

const DB: &str = "notes.db";

pub fn create_notes_table() -> Result<()> {
    let db = Connection::open(&DB[..])?;
    db.execute(
        "CREATE TABLE IF NOT EXISTS notes (id TEXT, title TEXT, contents TEXT)",
        NO_PARAMS,
    )?;
    Ok(())
}
