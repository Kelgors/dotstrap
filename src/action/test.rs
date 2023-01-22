use crate::{
    hashmap,
    host::config::{PackageManager, PackageManagerCommands},
};

use super::*;

#[test]
fn test_compacter_adjoining_packages() {
    let sysactions = vec![
        SystemAction::Package {
            operation: PackageOperation::Uninstall,
            source: "os".to_string(),
            name: "bash".to_string(),
            origin: "packages/bash".to_string(),
        },
        SystemAction::Package {
            operation: PackageOperation::Uninstall,
            source: "os".to_string(),
            name: "bash-completion".to_string(),
            origin: "packages/bash".to_string(),
        },
        SystemAction::Package {
            operation: PackageOperation::Uninstall,
            source: "os".to_string(),
            name: "fish".to_string(),
            origin: "hosts/Kelgors-Desktop".to_string(),
        },
        SystemAction::Package {
            operation: PackageOperation::Install,
            source: "os".to_string(),
            name: "zsh".to_string(),
            origin: "packages/zsh".to_string(),
        },
        SystemAction::Package {
            operation: PackageOperation::Install,
            source: "os".to_string(),
            name: "zsh-syntax-highlighting".to_string(),
            origin: "packages/zsh".to_string(),
        },
        SystemAction::Package {
            operation: PackageOperation::Install,
            source: "os".to_string(),
            name: "zsh-autosuggestions".to_string(),
            origin: "packages/zsh".to_string(),
        },
        SystemAction::Package {
            operation: PackageOperation::Install,
            source: "os".to_string(),
            name: "zsh-history-substring-search".to_string(),
            origin: "packages/zsh".to_string(),
        },
        SystemAction::Package {
            operation: PackageOperation::Install,
            source: "os".to_string(),
            name: "zsh-theme-powerlevel10k".to_string(),
            origin: "packages/zsh".to_string(),
        },
        SystemAction::File {
            operation: FileOperation::Link,
            src: "zshrc".to_string(),
            dest: "~/.zshrc".to_string(),
            origin: "packages/zsh".to_string(),
        },
        SystemAction::File {
            operation: FileOperation::Copy,
            src: "profile".to_string(),
            dest: "~/.$USER.profile".to_string(),
            origin: "packages/zsh".to_string(),
        },
    ];

    let config = HostConfiguration {
        package_managers: hashmap![
            "os".to_string() => PackageManager {
                multiple: true,
                commands: PackageManagerCommands {
                    install: "paru -S <package>".to_string(),
                    uninstall: "paru -Runs <package>".to_string(),
                    clean: None,
                },
            }
        ],
    };

    let result = vec![
            SystemAction::Package {
                operation: PackageOperation::Uninstall,
                source: "os".to_string(),
                name: "bash bash-completion".to_string(),
                origin: "packages/bash".to_string(),
            },
            SystemAction::Package {
                operation: PackageOperation::Uninstall,
                source: "os".to_string(),
                name: "fish".to_string(),
                origin: "hosts/Kelgors-Desktop".to_string(),
            },
            SystemAction::Package {
                operation: PackageOperation::Install,
                source: "os".to_string(),
                name: "zsh zsh-syntax-highlighting zsh-autosuggestions zsh-history-substring-search zsh-theme-powerlevel10k".to_string(),
                origin: "packages/zsh".to_string(),
            },
            SystemAction::File {
                operation: FileOperation::Link,
                src: "zshrc".to_string(),
                dest: "~/.zshrc".to_string(),
                origin: "packages/zsh".to_string(),
            },
            SystemAction::File {
                operation: FileOperation::Copy,
                src: "profile".to_string(),
                dest: "~/.$USER.profile".to_string(),
                origin: "packages/zsh".to_string(),
            },
        ];
    let merged_actions = compact_mergeable_actions(&sysactions, &config);
    assert_eq!(result, merged_actions);
}
