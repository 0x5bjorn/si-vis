mod app;
mod si_data;
mod tui;
mod ui;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::{io, time::Duration};

fn main() -> app::AppResult<()> {
    let mut app = app::App::new();

    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let mut tui = tui::Tui::new(terminal);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('a') => app.next_tab_index(),
                        KeyCode::Char('d') => app.previous_tab_index(),
                        _ => {}
                    }
                }
            }
        }
    }
    tui.exit()?;

    app.quit();
    Ok(())
}
