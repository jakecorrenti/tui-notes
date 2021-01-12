use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .margin(1)
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    let notes_list = Block::default().borders(Borders::ALL).title("Notes");
    f.render_widget(notes_list, chunks[0]);

    draw_current_note_panel(f, &chunks[1]);
}

fn draw_current_note_panel<B: Backend>(f: &mut Frame<B>, layout_chunk: &Rect) {
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(90),
            ]
            .as_ref(),
        )
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
