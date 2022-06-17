use std::path::Path;

use git2::{Repository, Index, ObjectType, Signature, Error, Direction, Cred, PushOptions};

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
    push(&repo)?;
    // let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    // let commit = obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))?;
    // println!("commit {}\nAuthor: {}\n    {}",
    //          commit.id(),
    //          commit.author(),
    //          commit.message().unwrap_or("no commit message"));
    Ok(())
}

fn open() -> Result<Repository, git2::Error>{
    let repo = Repository::open_from_env().unwrap();
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
    let signature = Signature::now("buhe", "bugu1986@126.com")?;
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
        println!("pull up to date");
        Ok(())
    } else if analysis.0.is_fast_forward() {
        println!("pull fast forward");
        let refname = format!("refs/heads/{}", "master");
        let mut reference = repo.find_reference(&refname)?;
        reference.set_target(fetch_commit.id(), "Fast-Forward")?;
        repo.set_head(&refname)?;
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
    } else {
        Err(Error::from_str("Fast-forward only!"))
    }
}


fn push(repo: &Repository) -> Result<(), git2::Error> {
    let mut remote = match repo.find_remote("origin") {
        Ok(r) => r,
        Err(_) => repo.remote("origin", "unknow")?,
    };
    // remote.connect(Direction::Push)?;
    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.credentials(git_credentials_callback);
    remote.connect_auth(Direction::Push, Some(callbacks), None)?;
    println!("connected.");  
    repo.remote_add_push("origin", "refs/heads/master:refs/heads/master").unwrap();

    let mut push_options = PushOptions::default();
    let mut callbacks2 = git2::RemoteCallbacks::new();
    callbacks2.credentials(git_credentials_callback);
    push_options.remote_callbacks(callbacks2);
    // remote.push(&["refs/heads/master:refs/heads/master"], None)
    remote.push(&["refs/heads/master:refs/heads/master"], Some(&mut push_options))
}

pub fn git_credentials_callback(
    _url: &str,
    _user_from_url: Option<&str>,
    _cred_types_allowed: git2::CredentialType,
) -> Result<git2::Cred, git2::Error> {
    

    // if cred_types_allowed.contains(git2::CredentialType::SSH_KEY) {
    //     // let user = user_from_url.unwrap();
    //    return Cred::ssh_key("buhe", Some(Path::new("~/.ssh/github.pub")), Path::new("~/.ssh/github"), None);
    // }

    // return Err(git2::Error::from_str(format!("no credential option available for {:#?} {:#?}", user_from_url, cred_types_allowed).as_str()));
    // println!("auth {} {:#?} {:#?}", _user, _user_from_url, _cred);
//    Cred::ssh_key_from_agent("buhe")
    //     let credentials = 
	// 	Cred::ssh_key_from_agent(
	// 		"github").expect("Could not create credentials object");


	// Ok(credentials)
    let p = env!("KEY");
    println!("pass is '{}'", p);
    Cred::userpass_plaintext("buhe", p)
}