use git2::{Commit, ObjectType, Repository};

fn main() {
    let repo = match Repository::init("/home/syien/rust/gterm") {
        Ok(repo) => repo,
        Err(e) => panic!("ERROR loading repository: {e}"),
    };
    let obj = repo
        .head()
        .unwrap()
        .resolve()
        .unwrap()
        .peel(ObjectType::Commit)
        .unwrap();
    let commit = obj.clone().into_commit().unwrap();
    let timestamp = commit.time().seconds();
    println!(
        "{commit_id}\n{commit_author}\n{timestamp}\n{commit_message}",
        commit_id = commit.id(),
        commit_author = commit.author(),
        commit_message = commit.message().unwrap()
    );
}
