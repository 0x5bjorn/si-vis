use crate::app::AppResult;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{Alignment, Backend},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};
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

    pub fn draw(&mut self) -> AppResult<()> {
        self.term.draw(|frame| {
            let greeting = Paragraph::new("Hello World!")
                .block(
                    Block::default()
                        .title("Template")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                .alignment(Alignment::Center);
            frame.render_widget(greeting, frame.size());
        })?;

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

// fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
//     let mut stdout = io::stdout();
//     enable_raw_mode()?;
//     execute!(stdout, EnterAlternateScreen)?;
//     Ok(Terminal::new(CrosstermBackend::new(stdout))?)
// }

// fn restore_terminal(
//     terminal: &mut Terminal<CrosstermBackend<Stdout>>,
// ) -> Result<(), Box<dyn Error>> {
//     disable_raw_mode()?;
//     execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
//     Ok(terminal.show_cursor()?)
// }

// fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
//     Ok(loop {
//         terminal.draw(|frame| {
//             let greeting = Paragraph::new("Hello World!");
//             frame.render_widget(greeting, frame.size());
//         })?;
//         if event::poll(Duration::from_millis(250))? {
//             if let Event::Key(key) = event::read()? {
//                 if KeyCode::Char('q') == key.code {
//                     break;
//                 }
//             }
//         }
//     })
// }
