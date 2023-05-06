use crossterm::{
    event::{self, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use git2::{Commit, ObjectType, Repository};
use std::{io, thread, time};
use tui::{backend::CrosstermBackend, Terminal};

fn main() {
    let repo = match Repository::init("/home/syien/rust/gterm") {
        Ok(repo) => repo,
        Err(e) => panic!("ERROR loading repository: {e}"),
    };
    let backend = CrosstermBackend::new(io::stdout());
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture).unwrap();
    let mut terminal = Terminal::new(backend).unwrap();
    display_last_commit(&repo).unwrap();
    thread::sleep(time::Duration::from_millis(5000));
    execute!(io::stdout(), LeaveAlternateScreen);
}

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    let commit = obj.into_commit().unwrap();
    return Ok(commit);
}

fn display_last_commit(repo: &Repository) -> Result<(), git2::Error> {
    let commit = find_last_commit(repo).expect("ERROR:Could not find last commit");
    let timestamp = commit.time().seconds();
    Ok(println!(
        "commit_id: {commit_id}\ncommit_author: {commit_author}\ntimestamp: {timestamp}\ncommit_message: {commit_message}",
        commit_id = commit.id(),
        commit_author = commit.author(),
        commit_message = commit.message().unwrap()
    ))
}
