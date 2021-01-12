extern crate rusqlite;
use rusqlite::{Connection, Result, NO_PARAMS};

use super::note::Note;

const DB: &str = "notes.db";

pub fn create_notes_table() -> Result<()> {
    let db = Connection::open(&DB[..])?;
    db.execute(
        "CREATE TABLE IF NOT EXISTS notes (id TEXT, title TEXT, contents TEXT)",
        NO_PARAMS,
    )?;
    Ok(())
}

pub fn insert_note(note: Note) -> Result<()> {
    let db = Connection::open(&DB[..])?;
    db.execute(
        "INSERT INTO notes (id, title, contents) VALUES (?1, ?2, ?3)",
        &[&note.id, &note.title, &note.contents],
    )?;
    Ok(())
}

pub fn get_note(id: String) -> Result<Note> {
    let db = Connection::open(&DB[..])?;
    let mut stmt = db.prepare("SELECT id, title, contents FROM notes WHERE id=(?1)")?;
    stmt.query_row(&[&id], |row| {
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            contents: row.get(2)?,
        })
    })
}

pub fn update_note(note: Note) -> Result<()> {
    let db = Connection::open(&DB[..])?;
    db.execute(
        "UPDATE notes SET id=(?1) title=(?2) contents=(?3) WHERE id=(?1)",
        &[&note.id, &note.title, &note.contents],
    )?;
    Ok(())
}

pub fn delete_note(note: Note) -> Result<()> {
    let db = Connection::open(&DB[..])?;
    db.execute("DELETE FROM notes WHERE id=(?1)", &[&note.id])?;
    Ok(())
}

pub fn get_all_notes() -> Result<Vec<Note>> {
    let db = Connection::open(&DB[..])?;

    let mut stmt = db.prepare("SELECT id, title, contents FROM notes")?;
    let notes = stmt.query_map(NO_PARAMS, |row| {
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            contents: row.get(2)?,
        })
    })?;

    let mut notes_vec = Vec::new();
    notes.for_each(|note| notes_vec.push(note.unwrap()));

    Ok(notes_vec)
}
