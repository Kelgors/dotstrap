use anyhow::Result;
use pathbuf::pathbuf;

use crate::action::FileOperation;
use crate::action::PackageOperation;
use crate::action::SystemAction;

fn make_difference<T: Clone + PartialEq>(from: &Vec<T>, to: &Vec<T>) -> Vec<T> {
    let mut difference = vec![];
    for item in from.into_iter() {
        if !to.contains(item) {
            difference.push(item.clone());
        }
    }
    return difference;
}

fn inverse_additive_actions(system_actions: &Vec<SystemAction>) -> Vec<SystemAction> {
    return system_actions
        .into_iter()
        .filter(|sysaction| match sysaction {
            SystemAction::Package {
                operation,
                source: _,
                name: _,
                origin: _,
            } => PackageOperation::Uninstall.ne(operation),
            SystemAction::File {
                operation,
                src: _,
                dest: _,
                origin: _,
            } => FileOperation::Remove.ne(operation),
            _ => false,
        })
        .map(|sysaction| match sysaction {
            SystemAction::Package {
                operation: _,
                source,
                name,
                origin,
            } => {
                return SystemAction::Package {
                    operation: PackageOperation::Uninstall,
                    source: source.clone(),
                    name: name.clone(),
                    origin: origin.clone(),
                };
            }
            SystemAction::File {
                operation: _,
                src: _,
                dest,
                origin,
            } => {
                return SystemAction::File {
                    operation: FileOperation::Remove,
                    src: String::new(),
                    dest: dest.clone(),
                    origin: origin.clone(),
                };
            }
            _ => panic!["Cannot reverse other actions than File & Package !"],
        })
        .rev()
        .collect();
}

pub fn build_action_diff(
    next_system_actions: &Vec<SystemAction>,
    full: bool,
) -> Result<Vec<SystemAction>> {
    let pwd = std::env::current_dir()?;
    let lockfile_path = pathbuf![&pwd, ".lockfile"];
    if !lockfile_path.exists() {
        return Ok(next_system_actions.clone());
    }
    let file_content = std::fs::read_to_string(&lockfile_path)?;
    let previous_actions: Vec<SystemAction> = serde_yaml::from_str(&file_content)?;

    let missing_last_actions = make_difference(&previous_actions, next_system_actions);
    let mut delete_actions = inverse_additive_actions(&missing_last_actions);
    let mut needed_actions = if full {
        next_system_actions.clone()
    } else {
        make_difference(next_system_actions, &previous_actions)
    };
    let mut all_actions = vec![];
    all_actions.append(&mut delete_actions);
    all_actions.append(&mut needed_actions);
    return Ok(all_actions);
}
