use std::path::PathBuf;

use anyhow::{anyhow, Result};
use git2::{Commit, Config, Direction, ObjectType, Oid, Repository, Signature};

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    return obj
        .into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit"));
}

fn generate_signature(repo: Option<&Repository>) -> Result<Signature> {
    let mut user_email: Option<String> = None;
    let mut user_name: Option<String> = None;

    if let Some(_repo) = repo {
        let local_config = _repo.config()?;
        if user_email.is_none() {
            user_email = local_config.get_string("user.email").ok();
        }
        if user_name.is_none() {
            user_name = local_config.get_string("user.name").ok();
        }
    }
    if let Ok(path) = Config::find_global() {
        println!("Load GlobalGitConfig: {}", path.to_string_lossy());
        let global_config = Config::open(&path)?;
        println!("{:#?}", global_config.get_string("user.email"));
        if user_email.is_none() {
            user_email = global_config.get_string("user.email").ok();
        }
        if user_name.is_none() {
            user_name = global_config.get_string("user.name").ok();
        }
    }
    if let Ok(path) = Config::find_system() {
        println!("Load SystemGitConfig: {}", path.to_string_lossy());
        let system_config = Config::open(&path)?;
        if user_email.is_none() {
            user_email = system_config.get_string("user.email").ok();
        }
        if user_name.is_none() {
            user_name = system_config.get_string("user.name").ok();
        }
    }

    if user_email.is_some() && user_name.is_some() {
        return Ok(Signature::now(
            user_name.unwrap().as_str(),
            user_email.unwrap().as_str(),
        )?);
    }
    return Err(anyhow!["Cannot find git config"]);
}

pub fn add_and_commit(repo: &Repository, path: &PathBuf, message: &str) -> Result<Oid> {
    let mut index = repo.index()?;
    index.add_path(path)?;
    let oid = index.write_tree()?;
    let signature = generate_signature(Some(repo))?;
    let parent_commit = find_last_commit(repo)?;
    let tree = repo.find_tree(oid)?;
    return Ok(repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&parent_commit],
    )?);
}

fn push(repo: &Repository) -> Result<(), git2::Error> {
    let mut remote = repo
        .find_remote("origin")
        .expect("Missing git repo remote origin");
    remote.connect(Direction::Push)?;
    let current_branch = repo.branches(None)?.find_map(|branch| {
        let _branch = &branch.as_ref().unwrap().0;
        if _branch.is_head() {
            return _branch.name().unwrap().unwrap().clone();
        }
        return None;
    });
    if let Some(branch_name) = current_branch {
        println!("Branch name: {}", branch_name);
        // remote.push(&["refs/heads/master:refs/heads/master"], None);
    }
    Ok(())
}
