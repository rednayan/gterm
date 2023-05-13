use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use git2::{Commit, Repository};
use std::{env, io};
use tui::{backend::CrosstermBackend, Terminal};
mod ui;

pub struct AppData {
    pub titles: Vec<String>,
    pub index: usize,
}

impl AppData {
    fn new() -> Self {
        AppData {
            titles: vec![
                "commit logs".to_string(),
                "TBD".to_string(),
                "TBD".to_string(),
            ],
            index: 0,
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
    let mut stdout = io::stdout();

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

    let appdata = AppData::new();
    ui::run_app(&mut terminal, appdata)?;

    // commit_logs(&repo);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn commit_logs(repo: &Repository) {
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    for oid in revwalk {
        let commit_oid = oid.unwrap();
        let commit = repo.find_commit(commit_oid).unwrap();
        display_commits(&commit).unwrap();
    }
}

fn display_commits(commit: &Commit) -> Result<(), git2::Error> {
    let timestamp = commit.time().seconds();
    Ok(println!(
        "commit_id: {commit_id}
        \ncommit_author: {commit_author}
        \ntimestamp: {timestamp}
        \ncommit_message: {commit_message}
       ",
        commit_id = commit.id(),
        commit_author = commit.author(),
        commit_message = commit.message().unwrap(),
    ))
}
