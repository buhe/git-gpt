use std::{path::Path, time};

use git2::{Repository, Index, ObjectType};

fn main() {
    println!("Hello, world!");
    match run() {
        Ok(()) => {}
        Err(e) => println!("error: {}", e),
    }
    
}

fn run() -> Result<(), git2::Error> {
    let repo = open()?;
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    let commit = obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))?;
    println!("commit {}\nAuthor: {}\n    {}",
             commit.id(),
             commit.author(),
             commit.message().unwrap_or("no commit message"));
    Ok(())
}

fn open() -> Result<Repository, git2::Error>{
    let repo = Repository::open(&Path::new("."))?;
    // let mut index = repo.index()?;
    // let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    // let commit = obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"));
    Ok(repo)
}

fn add() {

}

fn commit() {

}

fn pull() {

}

fn push() {

}