use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use git2::{Commit, ObjectType, Repository};
use std::env;
use std::{io, thread, time};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};

fn main() {
    let mut args = env::args();
    args.next();
    let path = args.next().expect("ERROR: please enter path to a folder");
    let repo = match Repository::init(path) {
        Ok(repo) => repo,
        Err(e) => panic!("ERROR loading repository: {e}"),
    };

    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    for oid in revwalk {
        let commit_oid = oid.unwrap();
        let commit = repo.find_commit(commit_oid).unwrap();
        display_commits(&commit).unwrap();
    }

    let backend = CrosstermBackend::new(io::stdout());
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture).unwrap();
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            let size = f.size();
            let block = Block::default().title("Block").borders(Borders::ALL);
            f.render_widget(block, size);
        })
        .unwrap();
    thread::sleep(time::Duration::from_millis(5000));
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
}

fn display_commits(commit: &Commit) -> Result<(), git2::Error> {
    let timestamp = commit.time().seconds();
    Ok(println!(
        "commit_id: {commit_id}\ncommit_author: {commit_author}\ntimestamp: {timestamp}\ncommit_message: {commit_message}",
        commit_id = commit.id(),
        commit_author = commit.author(),
        commit_message = commit.message().unwrap()
    ))
}
