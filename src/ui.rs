use crate::app;

use ratatui::{prelude::*, widgets::*, Frame};
use std::rc::Rc;

pub fn setup_layout<B: Backend>(frame: &mut Frame<'_, B>) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.size())
}

pub fn render_tabs<B: Backend>(
    app: &mut app::App,
    frame: &mut Frame<'_, B>,
    layout_chunks: Rc<[Rect]>,
) {
    let block = Block::default().on_white().black();
    frame.render_widget(block, frame.size());

    let titles = app.tab_titles.clone();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.tab_index)
        .style(Style::default().cyan().on_gray())
        .highlight_style(Style::default().bold().on_black());
    frame.render_widget(tabs, layout_chunks[0]);

    match app.tab_index {
        0 => frame.render_widget(
            Block::default().title("Inner 1").borders(Borders::ALL),
            layout_chunks[1],
        ),
        1 => render_table(app, frame, layout_chunks),
        _ => unreachable!(),
    };
}

pub fn render_table<B: Backend>(
    app: &mut app::App,
    frame: &mut Frame<'_, B>,
    layout_chunks: Rc<[Rect]>,
) {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = ["Header1", "Header2", "Header3"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);

    let items = vec![
        vec!["Row11", "Row12", "Row13"],
        vec!["Row21", "Row22", "Row23"],
        vec!["Row31", "Row32", "Row33"],
        vec!["Row41", "Row42", "Row43"],
        vec!["Row51", "Row52", "Row53"],
        vec!["Row61", "Row62\nTest", "Row63"],
        vec!["Row71", "Row72", "Row73"],
        vec!["Row81", "Row82", "Row83"],
        vec!["Row91", "Row92", "Row93"],
        vec!["Row101", "Row102", "Row103"],
        vec!["Row111", "Row112", "Row113"],
        vec!["Row121", "Row122", "Row123"],
        vec!["Row131", "Row132", "Row133"],
        vec!["Row141", "Row142", "Row143"],
        vec!["Row151", "Row152", "Row153"],
        vec!["Row161", "Row162", "Row163"],
        vec!["Row171", "Row172", "Row173"],
        vec!["Row181", "Row182", "Row183"],
        vec!["Row191", "Row192", "Row193"],
    ];

    let rows = items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(*c));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Table"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Max(30),
            Constraint::Min(10),
        ]);
    frame.render_widget(t, layout_chunks[1]);
}
