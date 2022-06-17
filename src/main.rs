use std::{path::Path};

use git2::{Repository, Index, ObjectType, Signature};

fn main() {
    println!("Hello, git!");
    match run() {
        Ok(()) => {}
        Err(e) => println!("error: {}", e),
    }
    
}

fn run() -> Result<(), git2::Error> {
    let repo = open()?;
    let mut index = add_all(&repo)?;
    commit(&repo, &mut index)?;
    // let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    // let commit = obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))?;
    // println!("commit {}\nAuthor: {}\n    {}",
    //          commit.id(),
    //          commit.author(),
    //          commit.message().unwrap_or("no commit message"));
    Ok(())
}

fn open() -> Result<Repository, git2::Error>{
    let repo = Repository::open(&Path::new("."))?;
    // let mut index = repo.index()?;
    // let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    // let commit = obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"));
    Ok(repo)
}

fn add_all(repo: &Repository) -> Result<Index, git2::Error> {
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    Ok(index)
}

fn commit(repo: &Repository, index: &mut Index) -> Result<(), git2::Error>  {
     let oid = index.write_tree()?;
    let signature = Signature::now("buhe", "bugu1986@gmail.com")?;
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    let parent_commit = obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))?;
    let tree = repo.find_tree(oid)?;
    let _commit = repo.commit(Some("HEAD"), //  point HEAD to our new commit
                &signature, // author
                &signature, // committer
                "git-u:update", // commit message
                &tree, // tree
                &[&parent_commit]); // parents

    Ok(())
}

fn pull() {

}

fn push() {

}