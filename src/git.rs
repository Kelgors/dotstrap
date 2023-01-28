use anyhow::Result;
use git2::{Commit, ObjectType, Oid, Repository};
use std::path::PathBuf;

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    return obj
        .into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit"));
}

pub fn add_and_commit(repo: &Repository, path: &PathBuf, message: &str) -> Result<Oid> {
    let tree_id = {
        let mut index = repo.index()?;
        println!("Adding {} to the git index", path.to_string_lossy());
        index.add_path(path)?;
        index.write_tree()?
    };
    let signature = repo.signature()?;
    let parent_commit = find_last_commit(repo)?;
    let tree = repo.find_tree(tree_id)?;
    let output = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&parent_commit],
    )?;
    // STRANGE:
    // At this point, commit is created with correct changes on package.yml
    // but it remains one staged version and one unstagged version
    // they cancel themselves when added to index or removed from index
    // The code below removes these artifacts, there is surely an underlying
    // issue but for now, i don't know how to resolve properly it.
    {
        let mut index = repo.index()?;
        index.add_path(path)?;
        index.write()?
    }

    println!("Successfuly committed as \"{}\"", message);
    return Ok(output);
}

pub fn push(repo: &Repository) -> Result<(), git2::Error> {
    let config = repo.config()?;
    let mut remote = repo
        .find_remote("origin")
        .expect("Missing git repo remote origin");

    let head = repo.head().unwrap();
    let branch_name = head.name().unwrap();

    let mut remote_callbacks = git2::RemoteCallbacks::new();
    remote_callbacks.credentials(|url, username, allowed| {
        let mut cred_helper = git2::CredentialHelper::new(url);
        cred_helper.config(&config);
        let creds = if allowed.contains(git2::CredentialType::SSH_KEY) {
            let user = username
                .map(|s| s.to_string())
                .or_else(|| cred_helper.username.clone())
                .unwrap_or("git".to_string());
            git2::Cred::ssh_key_from_agent(&user)
        } else if allowed.contains(git2::CredentialType::USER_PASS_PLAINTEXT) {
            git2::Cred::credential_helper(&config, url, username)
        } else if allowed.contains(git2::CredentialType::DEFAULT) {
            git2::Cred::default()
        } else {
            Err(git2::Error::from_str("no authentication available"))
        };
        return creds;
    });

    let mut push_options = git2::PushOptions::new();
    push_options.remote_callbacks(remote_callbacks);

    remote
        .push(
            &[format!("{}:{}", &branch_name, &branch_name)],
            Some(&mut push_options),
        )
        .expect("Cannot push");
    println!("Successfuly pushed to {}", &branch_name[11..]);
    return Ok(());
}
