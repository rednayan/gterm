use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use git2::Commit;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame, Terminal,
};

use crate::AppData;

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut appdata: AppData,
    commit_logs: Vec<Commit>,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &appdata, commit_logs.clone()))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right => appdata.next(),
                KeyCode::Left => appdata.previous(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, appdata: &AppData, commit_logs: Vec<Commit>) {
    let terminal_size = f.size();
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(terminal_size);

    let blocks = Block::default().style(
        Style::default()
            .bg(tui::style::Color::Black)
            .fg(tui::style::Color::White),
    );
    f.render_widget(blocks, terminal_size);
    let titles = appdata
        .titles
        .iter()
        .map(|t| {
            let (first, last) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(last, Style::default().fg(Color::Green)),
            ])
        })
        .collect();

    let commit = commit_logs.iter().map(|f| {
        Span::styled(f.author().to_string(), Style::default().fg(Color::Magenta));
    });

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(appdata.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::White),
        );
    f.render_widget(tabs, terminal_size);

    let inner = match appdata.index {
        0 => Block::default().title("logs").borders(Borders::ALL),
        1 => Block::default().title("Inner 1").borders(Borders::ALL),
        2 => Block::default().title("Inner 2").borders(Borders::ALL),
        3 => Block::default().title("Inner 3").borders(Borders::ALL),
        _ => unreachable!(),
    };
    f.render_widget(inner, chunks[1]);
    let paragraph = Paragraph::new(vec![Spans::from("this is a para")])
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .alignment(tui::layout::Alignment::Left);

    f.render_widget(paragraph, chunks[0]);
}
