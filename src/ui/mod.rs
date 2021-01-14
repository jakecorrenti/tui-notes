use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use super::{db, note::Note, notes_list_events::NoteListEvents, NoteState};

pub fn draw<B: Backend>(
    f: &mut Frame<B>,
    list_state: &mut NoteListEvents,
    note_state: &mut NoteState,
) {
    let chunks = Layout::default()
        .margin(1)
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    draw_notes_list(f, &chunks[0], list_state);

    draw_current_note_panel(f, &chunks[1], list_state, note_state);
}

fn draw_current_note_panel<B: Backend>(
    f: &mut Frame<B>,
    layout_chunk: &Rect,
    list_state: &mut NoteListEvents,
    note_state: &mut NoteState,
) {
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(*layout_chunk);

    draw_current_note_title(f, parent_layout[0], list_state);
    draw_current_note_contents(f, parent_layout[1], list_state, note_state);
}

fn draw_current_note_title<B: Backend>(
    f: &mut Frame<B>,
    layout_chunk: Rect,
    list_state: &mut NoteListEvents,
) {
    let mut text: Vec<Span> = Vec::new();
    let note: Note;

    if let Some(id) = list_state.selected_note_id() {
        note = db::get_note(id).expect("Unable to access the current selected note");
        note.title
            .lines()
            .for_each(|line| text.push(Span::raw(line)));
    }

    let paragraph_contents = vec![Spans::from(text)];

    let paragraph = Paragraph::new(paragraph_contents)
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::ALL).title("Title"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, layout_chunk);
}

fn draw_current_note_contents<B: Backend>(
    f: &mut Frame<B>,
    layout_chunk: Rect,
    list_state: &mut NoteListEvents,
    note_state: &mut NoteState,
) {
    let available_width_for_text = (layout_chunk.width - 2) as usize;
    let mut text: Vec<Span> = Vec::new();
    let note: Note;

    if let Some(id) = list_state.selected_note_id() {
        note = db::get_note(id).expect("Unable to access the current selected note");
        note.contents
            .lines()
            .for_each(|line| text.push(Span::raw(line)));
    }

    let paragraph_contents = vec![Spans::from(text)];
    let paragraph = Paragraph::new(paragraph_contents)
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::ALL).title("Content"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, layout_chunk);

    // current cursor location is incorrect because the note_state and the list_state are not
    // synced
    let cursor = note_state.cursor_loc(available_width_for_text);
    f.set_cursor(layout_chunk.x + 1 + cursor.0, layout_chunk.y + 1 + cursor.1);
}

fn draw_notes_list<B: Backend>(
    f: &mut Frame<B>,
    layout_chunk: &Rect,
    list_state: &mut NoteListEvents,
) {
    let notes = db::get_all_notes().expect("There was an error retrieving your notes");

    notes
        .iter()
        .for_each(|note| list_state.add_note(note.id.clone(), note.title.clone()));

    let list_items: Vec<ListItem> = list_state
        .notes
        .iter()
        .map(|note| ListItem::new(&note.1[..]))
        .collect();

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title("Notes"))
        .highlight_symbol("> ")
        .highlight_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, *layout_chunk, &mut list_state.state);
}
