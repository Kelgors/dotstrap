use anyhow::Result;
use pathbuf::pathbuf;
use promptly::prompt_default;

use crate::{
    action::{
        compact_mergeable_actions, execution::execute, transform_package_to_actions, SystemAction,
    },
    host::HostDefinition,
    lockfile::build_action_diff,
    resolver,
};

pub struct RunInstallOptions {
    pub dry: bool,
    pub full: bool,
}

pub fn run_install(hostname: String, options: RunInstallOptions) -> Result<()> {
    // Load host definition and prepare system actions from it
    let host_definition = HostDefinition::from_path(&pathbuf!["hosts", &hostname])?;
    let packages_repo = resolver::resolve_dependencies(&host_definition.package)?;
    let next_system_actions =
        transform_package_to_actions(&host_definition.package, &packages_repo, &mut vec![])?;
    // save next lockfile before "cleaning" mutation
    let serialized_next_lockfile: String = serde_yaml::to_string(&next_system_actions)?;
    // merge next actions with cleaning actions
    let all_actions: Vec<SystemAction> = build_action_diff(&next_system_actions, options.full)?;
    // compacting actions when possible
    let merged_actions = compact_mergeable_actions(&all_actions, &host_definition.config);

    if options.dry {
        println!("DryMode: {}", options.dry);
    }
    let confirm_execution = if !options.dry {
        prompt_default(
            format!("Do you want to apply {} operations?", all_actions.len()),
            false,
        )?
    } else {
        true
    };

    if confirm_execution {
        execute(&merged_actions, &host_definition.config, options.dry)?;
        if !options.dry {
            std::fs::write(
                pathbuf![&std::env::current_dir()?, ".lockfile"],
                &serialized_next_lockfile,
            )?;
        }
    }
    return Ok(());
}
