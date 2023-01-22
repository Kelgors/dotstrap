use anyhow::{Context, Result};
use pathbuf::pathbuf;
use std::collections::HashMap;

use crate::package::PackageDefinition;

pub fn resolve_dependencies(
    package: &PackageDefinition,
) -> Result<HashMap<String, PackageDefinition>> {
    let mut dependencies_map = HashMap::<String, PackageDefinition>::new();
    let package_deps = &package.dependencies;

    for dependency in package_deps.into_iter() {
        let dep_src = &dependency.source;
        if dep_src != "dot" {
            continue;
        }
        let dep_name = dependency.name.clone();
        // Load package
        let package_pathname = pathbuf!["packages", &dep_name, "package.yml"];
        let definition = PackageDefinition::load(&package_pathname).context(format!(
            "Unable to parse {}",
            package_pathname.to_str().unwrap()
        ))?;
        dependencies_map.extend(resolve_dependencies(&definition)?);
        dependencies_map.insert(dep_name, definition);
    }
    return Ok(dependencies_map);
}
