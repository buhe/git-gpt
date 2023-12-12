mod sdk;

use git2::{Repository, Index, ObjectType};
use sdk::GPT;
use std::process::Command;
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

async fn commit(repo: &Repository, index: &mut Index, skip: bool, verbose: bool) -> Result<(), git2::Error> {
    let oid = index.write_tree()?;
    let signature = repo.signature()?;
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    let parent_commit = obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))?;
    let mut msg: String = "git-gpt:update".to_string();
    if !skip {
     let output = Command::new("git")
         .args(&["diff", "--cached", "--", ".", ":!*.lock", ":!.vscode/", ":!LICENSE", ":!*.xcbkptlist", ":!*.xcuserstate", ":!package-lock.json", ":!*.plist", ":!*.pbxproj"])
         .output()
         .expect("Failed to execute git command");
        let mut result = String::from_utf8_lossy(&output.stdout).to_string();
        if verbose {
            println!("git diff {}", result);
        }
        let mut gpt = GPT::new();
        if result.is_empty() {
            println!("All files are skip.");
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

