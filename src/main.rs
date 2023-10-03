mod app;
mod tui;
mod ui;
mod sys_data;

use app::{App, AppResult};
use tui::Tui;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::{io, time::Duration};

fn main() -> AppResult<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;

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
