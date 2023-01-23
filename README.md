# dotstrap

dotstrap allows you to manage your dotfiles and package them with the related application. dotstrap is between ansible and an AUR package, in a lot simplier form. You can create your own packages (set of dependencies, files and script), link them together, create a configuration for one or more computers and keep all of that stuff on one git repository.

An installation will generate a lockfile, or the *what-you-ve-done-the-last-time.lock* file. This will permit when you alter your configuration and run install again, to remove unnecessary installed packages or linked files from your last installation.

As long as your host config.yml is well configured and you're on a linux-based OS, it *should* work.

**This application is not ready for production, use it at your own risks**

## Getting Started

### Installation

```sh
git clone https://github.com/Kelgors/dotstrap.git
cd dotstrap
cargo install --path .
```

### Usage

```sh
mkdir ~/.dotstrap
cd ~/.dotstrap
# Generate the basic file tree
dotstrap init
# Applying configuration
dotstrap install
# This is the same as 
dotstrap install $(cat /etc/hostname)
# You can also use the --dry flag to know what dotstrap
# will do without altering your system.
dotstrap install --dry
```

## Make your own configuration

### File structure

```
packages/
    base/
        vimrc        # Your custom vim config file
        package.yml  # dotstrap package definition
host/
    alfred/
        config.yml   # Your host configuration
        package.yml  # Your host package definition
```

### packages/base/package.yml

This represents your dot:base package.

```yml
# package.yml
description: My basics for each setup
# Dependencies can be system packages or dot packages
# If you don't prefix the package name, this will be a system package. 
# dot packages are prefixed with "dot:"
# system packages are prefixed with "system:"
dependencies:
  - sudo
  - vim
  - curl
  - git
  - openssh
  - man-db
# You can run something after dependencies are installed
post_install: |
  sudo systemctl enable sshd
# You can also link/copy some files if needed
links:
  - src: vimrc      # the src is taken in package directory
    dest: ~/.vimrc  # the dest should be an absolute path
```

### host/alfred/package.yml

if your computer hostname is alfred, this will work out of the box.

```yml
# package.yml
description: My host setup
# What do you need in terms of system packages
dependencies:
  - dot:base    # this will load the packages/base/package.yml
  - rust        # this will install rust via the system package defined in config.yml
```
### host/alfred/config.yml

```yml
# config.yml
# This is a simple map referencing your packages managers
# You can add as many as you want but "os" is a reserved word
# for you system packages, in this case, os package manager will
# be "paru". But you can use what you want:
# apt, pacman, yay, dnf, ...
# if you want to specify flatpak as your os package manager, you can.
package_managers: 
  os:
    multiple: true # if the package manager can handle multiple package operation at once
    commands:
      install: paru --needed --noconfirm -S <package>
      uninstall: paru -Runs --noconfirm <package>

```

### More advanced examples

[https://github.com/Kelgors/dotpackages](https://github.com/Kelgors/dotpackages) using paru (arch) and flatpak in Kelgors-Desktop host.

