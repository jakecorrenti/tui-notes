use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use super::{db, note::Note, AppState};

pub fn draw<B: Backend>(f: &mut Frame<B>, state: &mut AppState) {
    let chunks = Layout::default()
        .margin(1)
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    draw_notes_list(f, &chunks[0], state);

    draw_current_note_panel(f, &chunks[1], state);
}

fn draw_current_note_panel<B: Backend>(
    f: &mut Frame<B>,
    layout_chunk: &Rect,
    state: &mut AppState,
) {
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(*layout_chunk);

    draw_current_note_title(f, parent_layout[0], state);
    draw_current_note_contents(f, parent_layout[1], state);
}

fn draw_current_note_title<B: Backend>(f: &mut Frame<B>, layout_chunk: Rect, state: &mut AppState) {
    let mut text: Vec<Span> = Vec::new();
    let note: Note;

    if let Some(id) = state.selected_note_id() {
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
    state: &mut AppState,
) {
    let available_width_for_text = (layout_chunk.width - 2) as usize;
    let mut text: Vec<Span> = Vec::new();
    let note: Note;

    if let Some(id) = state.selected_note_id() {
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
    let cursor_loc = state.content_cursor_loc(available_width_for_text);

    // takes the coordinate of the frame, adds the cursor position in the respective direction,
    // then adds 1 to account for the space where the next character will be entered
    let x_offset = layout_chunk.x + cursor_loc.0 + 1;
    let y_offset = layout_chunk.y + cursor_loc.1 + 1;
    f.set_cursor(x_offset, y_offset);
}

fn draw_notes_list<B: Backend>(f: &mut Frame<B>, layout_chunk: &Rect, state: &mut AppState) {
    let notes = db::get_all_notes().expect("There was an error retrieving your notes");

    state.notes = notes;

    let list_items: Vec<ListItem> = state
        .notes
        .iter()
        .map(|note| ListItem::new(&note.title[..]))
        .collect();

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title("Notes"))
        .highlight_symbol("> ")
        .highlight_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, *layout_chunk, &mut state.list_state);
}
