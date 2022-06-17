use std::{path::Path};

use git2::{Repository, Index, ObjectType, Signature, Error};

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
    pull(&repo)?;
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

fn commit(repo: &Repository, index: &mut Index) -> Result<(), git2::Error> {
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

fn pull(repo: &Repository) -> Result<(), git2::Error> {
    repo.find_remote("origin")?
        .fetch(&["master"], None, None)?;

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
    let analysis = repo.merge_analysis(&[&fetch_commit])?;
    if analysis.0.is_up_to_date() {
        Ok(())
    } else if analysis.0.is_fast_forward() {
        let refname = format!("refs/heads/{}", "master");
        let mut reference = repo.find_reference(&refname)?;
        reference.set_target(fetch_commit.id(), "Fast-Forward")?;
        repo.set_head(&refname)?;
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
    } else {
        Err(Error::from_str("Fast-forward only!"))
    }
}

fn push() {

}