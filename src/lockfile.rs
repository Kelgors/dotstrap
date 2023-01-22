use anyhow::Result;
use pathbuf::pathbuf;

use crate::action::FileOperation;
use crate::action::PackageOperation;
use crate::action::SystemAction;

pub fn get_cleaning_actions(
    next_system_actions: &Vec<SystemAction>,
) -> Result<Option<Vec<SystemAction>>> {
    let pwd = std::env::current_dir()?;
    let lockfile_path = pathbuf![&pwd, ".lockfile"];
    if !lockfile_path.exists() {
        return Ok(None);
    }
    let file_content = std::fs::read_to_string(&lockfile_path)?;
    let previous_actions: Vec<SystemAction> = serde_yaml::from_str(&file_content)?;

    let mut difference = vec![];
    for prev_sys_action in (&previous_actions).into_iter() {
        if !next_system_actions.contains(&prev_sys_action) {
            difference.push(prev_sys_action);
        }
    }
    if difference.len() == 0 {
        return Ok(None);
    }
    return Ok(Some(
        (&difference)
            .into_iter()
            .filter(|sysaction| match sysaction {
                SystemAction::Package {
                    operation,
                    source: _,
                    name: _,
                } => PackageOperation::Install.eq(operation),
                SystemAction::File {
                    operation,
                    src: _,
                    dest: _,
                } => FileOperation::Remove.ne(operation),
                _ => false,
            })
            .map(|sysaction| match sysaction {
                SystemAction::Package {
                    operation: _,
                    source,
                    name,
                } => {
                    return SystemAction::Package {
                        operation: PackageOperation::Uninstall,
                        source: source.clone(),
                        name: name.clone(),
                    };
                }
                SystemAction::File {
                    operation: _,
                    src: _,
                    dest,
                } => {
                    return SystemAction::File {
                        operation: FileOperation::Remove,
                        src: String::new(),
                        dest: dest.clone(),
                    };
                }
                _ => panic!["Filtered values should not be mapped"],
            })
            .collect(),
    ));
}
