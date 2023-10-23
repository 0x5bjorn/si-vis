use crate::{app, si_data};

use ratatui::{prelude::*, widgets::*, Frame};
use std::rc::Rc;
use sysinfo::{ProcessExt, SystemExt};

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
        .block(Block::default().borders(Borders::ALL))
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
    let header_cells = [
        "PID",
        "Name",
        "CPU usage",
        "Memory",
        "Virtual memory",
        "Parent PID",
        "Runtime",
        "Disk usage",
    ]
    .iter()
    .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);

    let mg_sys_info = app.si_data.lock().unwrap();
    let items: Vec<Vec<String>> = mg_sys_info
        .sys_info
        .processes()
        .iter()
        .map(|(pid, proc)| {
            vec![
                pid.to_string(),
                proc.name().to_owned(),
                proc.cpu_usage().to_string(),
                si_data::to_gb(proc.memory() as u128),
                si_data::to_gb(proc.virtual_memory() as u128),
                match proc.parent() {
                    Some(pid) => pid.to_string(),
                    None => "None".to_owned(),
                },
                proc.run_time().to_string(),
                format!("{:?}", proc.disk_usage()),
            ]
        })
        .collect();

    let rows = items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(c.as_str()));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Processes"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(5),
            Constraint::Percentage(15),
            Constraint::Percentage(5),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(40),
        ]);
    frame.render_stateful_widget(t, layout_chunks[1], &mut app.table_state);
}
