use crate::app::{App, AppResult};
use crate::ui;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, Terminal};
use std::{io, panic};

pub struct Tui<B: Backend> {
    term: Terminal<B>,
}

impl<B: Backend> Tui<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        Self { term: terminal }
    }

    pub fn init(&mut self) -> AppResult<()> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;

        panic::set_hook(Box::new(|_| {
            Self::reset().expect("failed to reset the terminal");
        }));

        self.term.hide_cursor()?;
        self.term.clear()?;

        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
        self.term.draw(|frame| ui::render_table(app, frame))?;

        Ok(())
    }

    pub fn exit(&mut self) -> AppResult<()> {
        Self::reset()?;
        self.term.show_cursor()?;
        Ok(())
    }

    fn reset() -> AppResult<()> {
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen)?;
        Ok(())
    }
}
