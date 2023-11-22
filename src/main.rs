mod sdk;

use git2::{Repository, Index, ObjectType};
use sdk::GPT;
use std::{process::Command};
const MAX_NUM: usize = 4000;
#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut skip = false;
    let mut verbose = false;
    if args.contains(&"--skip".to_string()) || args.contains(&"-s".to_string()) {
        skip = true;
    }
    if args.contains(&"--verbose".to_string()) || args.contains(&"-v".to_string()) {
        verbose = true;
    }
    println!("Hello, git gpt!");
    match run(skip, verbose).await {
        Ok(()) => {}
        Err(e) => println!("error: {}", e),
    }
    
}

async fn run(skip: bool, verbose: bool) -> Result<(), git2::Error> {
    let repo = open()?;
    let mut index = add_all(&repo)?;
    commit(&repo, &mut index, skip, verbose).await?;
    // pull(&repo)?;
    // push(&repo)?;
    Ok(())
}

fn open() -> Result<Repository, git2::Error>{
    let repo = Repository::open_from_env().unwrap();
    Ok(repo)
}

fn add_all(repo: &Repository) -> Result<Index, git2::Error> {
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    Ok(index)
}

// fn skip_diff(mut command: Command) -> Command {
//     command
//     .arg("diff")
//         .arg("--cached");
//         // .arg("--")
//         // .arg(".");
//     // for arg in vec!["':!.vscode' ':!*.lock'", "':!LICENSE'", "':!*.xcbkptlist'", "':!*.xcuserstate'", "':!package-lock.json'", "':!*.plist'", "':!*.xcbkptlist"].into_iter() {
//     //     command.arg(arg);
//     // }
//     command
// }

async fn commit(repo: &Repository, index: &mut Index, skip: bool, verbose: bool) -> Result<(), git2::Error> {
    let oid = index.write_tree()?;
    let signature = repo.signature()?;
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    let parent_commit = obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))?;
    let mut msg: String = "git-gpt:update".to_string();
    if !skip {
     let output = Command::new("git")
         .args(&["diff", "--cached", ":!*.lock"])
         .output()
         .expect("Failed to execute git command");
        let mut result = String::from_utf8_lossy(&output.stdout).to_string();
        if verbose {
            println!("git diff {}", result);
        }
        let mut gpt = GPT::new();
        if result.is_empty() {
            msg = "All files are skip.".to_string();
            println!("GPT 3.5 API generate git commit log:{}", &msg);
        } else {
            if gpt.setup() {
                if result.len() >  MAX_NUM {
                    result.truncate(MAX_NUM)
                }
                let reps = gpt.request(result, verbose).await;
                if reps.is_err() {
                    println!("GPT 3.5 API {}.", reps.err().unwrap());
                    return Ok(());
                }
                msg = reps.unwrap();
                println!("GPT 3.5 API generate git commit log:{}", &msg);
                
            } else {
                return Ok(());
            }

        }
    }
    let tree = repo.find_tree(oid)?;
    // tree.as_object().as_commit().unwrap().message().unwrap();
    let _commit = repo.commit(Some("HEAD"), //  point HEAD to our new commit
                &signature, // author
                &signature, // committer
                &msg, // commit message
                &tree, // tree
                &[&parent_commit]); // parents
    Ok(())
}

// fn pull(repo: &Repository) -> Result<(), git2::Error> {
//     repo.find_remote("origin")?
//         .fetch(&["master"], None, None)?;

//     let fetch_head = repo.find_reference("FETCH_HEAD")?;
//     let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
//     let analysis = repo.merge_analysis(&[&fetch_commit])?;
//     if analysis.0.is_up_to_date() {
//         println!("pull up to date");
//         Ok(())
//     } else if analysis.0.is_fast_forward() {
//         println!("pull fast forward");
//         let refname = format!("refs/heads/{}", "master");
//         let mut reference = repo.find_reference(&refname)?;
//         reference.set_target(fetch_commit.id(), "Fast-Forward")?;
//         repo.set_head(&refname)?;
//         repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
//     } else {
//         Err(Error::from_str("Fast-forward only!"))
//     }
// }


// fn push(repo: &Repository) -> Result<(), git2::Error> {
//     let mut remote = match repo.find_remote("origin") {
//         Ok(r) => r,
//         Err(_) => repo.remote("origin", "unknow")?,
//     };
//     // remote.connect(Direction::Push)?;
//     // let mut callbacks = git2::RemoteCallbacks::new();
//     // callbacks.credentials(git_credentials_callback);
//     // remote.connect_auth(Direction::Push, Some(callbacks), None)?;
//     println!("connected.");  
//     // repo.remote_add_push("origin", "refs/heads/master:refs/heads/master").unwrap();

//     let mut push_options = PushOptions::default();
//     let mut callbacks2 = git2::RemoteCallbacks::new();
//     callbacks2.credentials(git_credentials_callback);
//     push_options.remote_callbacks(callbacks2);
//     // remote.push(&["refs/heads/master:refs/heads/master"], None)
//     remote.push(&["refs/heads/master:refs/heads/master"], Some(&mut push_options))
// }

// pub fn git_credentials_callback(
//     _url: &str,
//     _user_from_url: Option<&str>,
//     _cred_types_allowed: git2::CredentialType,
// ) -> Result<git2::Cred, git2::Error> {
    

//     // if cred_types_allowed.contains(git2::CredentialType::SSH_KEY) {
//     //     // let user = user_from_url.unwrap();
//     //    return Cred::ssh_key("buhe", Some(Path::new("~/.ssh/github.pub")), Path::new("~/.ssh/github"), None);
//     // }

//     // return Err(git2::Error::from_str(format!("no credential option available for {:#?} {:#?}", user_from_url, cred_types_allowed).as_str()));
//     // println!("auth {} {:#?} {:#?}", _user, _user_from_url, _cred);
// //    Cred::ssh_key_from_agent("buhe")
//     //     let credentials = 
// 	// 	Cred::ssh_key_from_agent(
// 	// 		"github").expect("Could not create credentials object");


// 	// Ok(credentials)
//     // let p = env!("KEY");
//     // println!("pass is '{}'", p);
//     Cred::userpass_plaintext("buhe", "")
// }