use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use git2::{Commit, Repository};
use std::env;
use tui::{
    backend::CrosstermBackend,
    widgets::{ListItem, ListState},
    Terminal,
};
mod ui;

pub struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub struct AppData<'a> {
    pub titles: Vec<String>,
    pub index: usize,
    pub items: StatefulList<Commit<'a>>,
}

impl<'a> AppData<'a> {
    fn new(items: StatefulList<Commit<'a>>) -> Self {
        AppData {
            titles: vec![
                "commit logs".to_string(),
                "TBD".to_string(),
                "TBD".to_string(),
            ],
            index: 0,
            items,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut args = env::args();
    args.next();
    let path = args.next().expect("ERROR: please enter path to a folder");
    let repo = match Repository::init(path) {
        Ok(repo) => repo,
        Err(e) => panic!("ERROR loading repository: {e}"),
    };
    let commit_logs: Vec<Commit> = commit_logs(&repo)?;

    let stateful_commit = StatefulList::with_items(commit_logs.clone());

    let appdata = AppData::new(stateful_commit);
    ui::run_app(&mut terminal, appdata)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn commit_logs(repo: &Repository) -> Result<Vec<Commit>> {
    let mut commit_vec: Vec<Commit> = Vec::new();
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    for oid in revwalk {
        let commit_oid = oid.unwrap();
        let commit = repo.find_commit(commit_oid).unwrap();
        commit_vec.push(commit);
    }
    Ok(commit_vec)
}
