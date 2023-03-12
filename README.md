# Dot File Manager

- DFM, acronym for Dot File Manager, is an easy-to-use application that has, as a primary objective, help you with the management of your dotfiles. It was thinked to work with one and only one remote repository. DFM uses Git to keep everything synchronized, so internet connection is needed if you are making changes to the repository.

## How DFM was made

- DFM is written in Rust, using [clap](https://crates.io/crates/clap) to make the command line parsing process and others crates like:
    - [colored](https://crates.io/crates/colored)
    - [online](https://crates.io/crates/online)
    - [regex](https://crates.io/crates/regex)
    - [thiserror](https://crates.io/crates/thiserror)
    - [walkdir](https://crates.io/crates/walkdir)

## How to use

### Add a file from the current directory to the remote repository

```
$ dfm add <file-name>
```

### List the files that are in the remote repository

```
$ dfm list
```

### Update a file that already is in the remote repository with a new version

```
$ dfm update <file-name>
```

### Remove a file from the remote repository

```
$ dfm remove <file-name>
```

### Show the the link of the remote repository that DFM is linked

```
$ dfm remote show
```

### Set the remote repository that DFM will synchronize with

```
$ dfm remote set <repository-ssh-link>
```

### Reset the DFM to the initial state (you will use that if you want to synchronize DFM with another repository)

```
$ dfm reset
```

### Clone a file from the remote repository to your current repository

```
$ dfm clone <file-name>
```

### Synchronize your repository with the remote repository (use that if your list command is out of date)

```
$ dfm sync
```

### Get DFM's current version

```
$ dfm --version
```

## How to install

- You can install DFM on your computer with cargo just typing ```cargo install dfm```. If you do not have cargo in your computer you can just follow this Rust [installation guide](https://www.rust-lang.org/tools/install).

## Troubleshooting

- If after the installation you can not execute DFM correctly in your terminal you can just open an issue and I will try to help.

## Uninstalling DFM

- Just type ```cargo uninstall dfm```.