use anyhow::Result;
use pathbuf::pathbuf;

use crate::{
    action::{
        compact_mergeable_actions, generation::generate_shell_script, transform_package_to_actions,
        SystemAction,
    },
    host::HostDefinition,
    lockfile::build_action_diff,
    resolver,
};

pub struct RunGenerateOptions {
    pub full: bool,
}

pub fn run_generate(hostname: String, options: RunGenerateOptions) -> Result<()> {
    let host_definition = HostDefinition::from_path(&pathbuf!["hosts", &hostname])?;
    let packages_repo = resolver::resolve_dependencies(&host_definition.package)?;
    let next_system_actions =
        transform_package_to_actions(&host_definition.package, &packages_repo, &mut vec![])?;
    // merge next actions with cleaning actions
    let all_actions: Vec<SystemAction> = build_action_diff(&next_system_actions, options.full)?;
    // compacting actions when possible
    let merged_actions = compact_mergeable_actions(&all_actions, &host_definition.config);
    // generate shell script
    let script = generate_shell_script(&merged_actions, &host_definition.config)?;
    println!("{}", script.join("\n"));
    return Ok(());
}
