use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use super::{db, notes_list_events::NoteListEvents};

pub fn draw<B: Backend>(f: &mut Frame<B>, list_state: &mut NoteListEvents) {
    let chunks = Layout::default()
        .margin(1)
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    draw_notes_list(f, &chunks[0], list_state);

    draw_current_note_panel(f, &chunks[1]);
}

fn draw_current_note_panel<B: Backend>(f: &mut Frame<B>, layout_chunk: &Rect) {
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(*layout_chunk);

    draw_current_note_title(f, parent_layout[0]);
    draw_current_note_contents(f, parent_layout[1]);
}

fn draw_current_note_title<B: Backend>(f: &mut Frame<B>, layout_chunk: Rect) {
    let text = vec![Spans::from(vec![Span::styled(
        "This is the title of the current note that was selected",
        Style::default().add_modifier(Modifier::BOLD),
    )])];
    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::ALL).title("Title"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, layout_chunk);
}

fn draw_current_note_contents<B: Backend>(f: &mut Frame<B>, layout_chunk: Rect) {
    let text = vec![Spans::from(vec![Span::raw(
        "This is the content of the current note that was selected",
    )])];
    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::ALL).title("Content"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, layout_chunk);
}

fn draw_notes_list<B: Backend>(f: &mut Frame<B>, layout_chunk: &Rect, list_state: &mut NoteListEvents) {

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
