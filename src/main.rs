use git2::{Commit, ObjectType, Repository};

fn main() {
    let repo = match Repository::init("/home/syien/rust/gterm") {
        Ok(repo) => repo,
        Err(e) => panic!("ERROR loading repository: {e}"),
    };

    display_last_commit(&repo);
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
