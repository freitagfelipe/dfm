# Dotfiles Manager

- dfmnn, is an easy-to-use application that has, as a primary objective, help you with the management of your dotfiles. It was thinked to work with one and only one remote repository. dfmn uses Git to keep everything synchronized, so internet connection is needed if you are using commands that make changes to the repository.

## How dfmnn was made

- dfmnn is written in Rust, using [clap](https://crates.io/crates/clap) to make the command line parsing process and others crates like:
    - [colored](https://crates.io/crates/colored)
    - [online](https://crates.io/crates/online)
    - [regex](https://crates.io/crates/regex)
    - [thiserror](https://crates.io/crates/thiserror)
    - [walkdir](https://crates.io/crates/walkdir)

## How to use

### Add a file from the current directory to the remote repository

```
$ dfmn add <file-name>
```

### List the files that are in the remote repository

```
$ dfmn list
```

### Update a file that already is in the remote repository with a new version

```
$ dfmn update <file-name>
```

### Remove a file from the remote repository

```
$ dfmn remove <file-name>
```

### Show the the link of the remote repository that dfmn is linked

```
$ dfmn remote show
```

### Set the remote repository that dfmn will synchronize with

```
$ dfmn remote set <repository-ssh-link>
```

### Reset the dfmn to the initial state (you will use that if you want to synchronize dfmn with another repository)

```
$ dfmn reset
```

### Clone a file from the remote repository to your current repository

```
$ dfmn clone <file-name>
```

### Synchronize your repository with the remote repository (use that if your list command is out of date)

```
$ dfmn sync
```

### Get dfmn's current version

```
$ dfmn --version
```

## How to install

- You can install dfmn on your computer with cargo just typing ```cargo install dfmn```. If you do not have cargo in your computer you can just follow this Rust [installation guide](https://www.rust-lang.org/tools/install).

## Troubleshooting

- If after the installation you can not execute dfmn correctly in your terminal you can just open an issue and I will try to help.

## Uninstalling dfmn

- Just type ```cargo uninstall dfmn```.