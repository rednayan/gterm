use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use git2::Commit;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Tabs},
    Frame, Terminal,
};

use crate::AppData;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut appdata: AppData) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut appdata))?;
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

fn ui<B: Backend>(f: &mut Frame<B>, appdata: &mut AppData) {
    let terminal_size = f.size();
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .margin(3)
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

    let commit_author: Vec<ListItem> = appdata
        .items
        .items
        .iter()
        .map(|f| {
            return ListItem::new(f.author().to_string());
        })
        .collect();

    let inner = match appdata.index {
        0 => list_block(commit_author.clone()),
        _ => unreachable!(),
    };
    f.render_stateful_widget(inner, chunks[1], &mut appdata.items.state);
}

fn list_block(commit_author: Vec<ListItem>) -> List {
    List::new(commit_author.clone())
        .block(Block::default().title("Commit List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
}
