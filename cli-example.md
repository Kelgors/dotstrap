[hostname] can be found via /etc/hostname

```sh
dotstrap
  init                # Initialize a dostrap repo
  validate [hostname] # show information about parsed packages
  generate [hostname] # generate applied actions as shell script
    --system            # only install packages
    --user              # only install dotfiles
    --root <path>       # root directory (default: /)
  install [hostname] # apply host configuration
    --system            # only install packages
    --user              # only install dotfiles
    --root <path>       # root directory (default: /)
    --repo github.com/Kelgors/dotfiles
    --verbose           # display logs
    --dry               # do everything without doing it
    --skip <type>       # can be one or many of os,flatpak,dot,link,script
```

```yaml
Models:
  - PackageDefinition
  - DependencyDefinition
  - HostDefinition
  - Action
    - type
    -
Utils:
  - ShellGenerationHelper
  - DependencyResolver
Runner:
  - ShellGenerationRunner
  - "ArchRunner = LinuxRunner(
      install: \"paru --needed -S <packages>\",
      uninstall: \"paru -Runs <packages>\"
    )"
```

```yaml
# config.yml
commands:
  update_database: "paru -Sy"
  upgrade_packages: "paru -Su"
  install_packages: "paru --needed -S <packages>"
  uninstall_packages: "paru -Runs <packages>"
```

```toml
[dependencies]
# cli
clap = "4.1.1"
# yaml parser
yaml-rust = "0.4.5"
```

Install command:
- Parse cli arguments
- Load config.yml
- Load host configuration
- Load dependencies
- Check if all dependencies are resolved
- transform packages in a set of actions
- execute set of actions

TODO:

config: os => default ?
skip option ?
.gitignore => .lockfile