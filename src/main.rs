mod app;
mod tui;

use std::{io, time::Duration};

use app::{App, AppResult};
use crossterm::event::{self, Event, KeyCode};
use tui::Tui;

use ratatui::{prelude::CrosstermBackend, Terminal};

fn main() -> AppResult<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);
    tui.init()?;

    while app.running {
        tui.draw()?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
            }
        }
    }
    tui.exit()?;

    app.quit();
    Ok(())
}
