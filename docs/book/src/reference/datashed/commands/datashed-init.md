# datashed-init(1)

## NAME

*datashed-init* --- Create a new datashed or re-initialize an existing
one

## SYNOPSIS

`datased init` [_OPTIONS_] [_DIRECTORY_]

## DESCRIPTION

This command creates a new datashed project that contains all the
necessary files and directories. If the command is run within an
existing datashed project, it simply reinitializes the project, meaning
that any missing artifacts are created.

If a `directory` is specified as an argument, it is created
automatically. Otherwise, the datashed is initialized in the current
directory.

Unless otherwise specified (`--vcs` option), the project is initialized
by default for use with [Git], and a corresponding `.gitignore` file
is created.

## ARGUMENTS

`DIRECTORY`
: The location of the new datashed (default `.`).

## OPTIONS

`-n`, `--name` `<NAME>`
: The name of the datashed.

`--version` `<VERSION>`
: The version of the datashed (default `0.1.0`).

`-d`, `--description` `<DESCRIPTION>`
: A short blurb about the datashed.

`-a`, `--author` `<AUTHOR>`
: A list of people or organizations, which are considered as the authors
of the datashed. By default the list is populated with the git identity
(if available).

`--vcs` `<VCS>`
: Initialize the datashed for the given version control system (VCS).
Possible values are `git` and `none`.

`-f`, `--force`
: Whether to overwrite config with default values or not.

`-h`, `--help`
: Print help

### COMMON OPTIONS

{{ #include common-opts.md }}


## EXIT STATUS

{{ #include exit-status.md }}

## EXAMPLES

The following command creates a new project `demo`:

```console
$ datashed init demo
```

Alternatively, a project can be initialized inside an existing
directory:

```console
$ mkdir demo && cd demo
$ datashed init .
```

[Git]: https://git-scm.com
