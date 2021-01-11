use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod ui;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    loop {
        terminal.draw(|f| {
            ui::draw(f);
        })?;
    }
}
