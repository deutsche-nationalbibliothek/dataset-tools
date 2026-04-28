# datashed-version(1)

## NAME

*datashed-version* --- Get or set the version of the datashed

## SYNOPSIS

`datashed version` [_OPTIONS_] [_VERSION_]

## DESCRIPTION

This command reads or updates the version of the datashed.

## ARGUMENTS

`VERSION`
: The new version of the dataset. Unless the `--force`/`-f` option is
set, the new version must be greater than the current version. A dataset
version consists of three separated integers, which must conform to the
[semantic versioning standard]; invalid version strings are rejected.

## OPTIONS

`-f`, `--force`
: Whether to overwrite the current version or not

`-b`, `--bump` `<BUMP[=VALUE]>`
: Update the dataset version using the given semantics. This option
conflicts with the `VERSION` argument. Possible values: `major`,
`minor`, `patch`.

`-h`, `--help`
: Print help

### COMMON OPTIONS

{{ #include common-opts.md }}

## EXIT STATUS

{{ #include exit-status.md }}

## EXAMPLES

The following command prints the current datashed version:

```console
$ datashed version
0.1.0
```

The version can be set to `0.1.1` using the following command:

```console
$ datashed version 0.1.1
```

Alternatively, the individual version components can be incremented
using the `--bump` option:

```console
$ datashed version --bump major # 0.1.0 -> 1.0.0
$ datashed version --bump minor # 0.1.0 -> 0.2.0
$ datashed version --bump patch # 0.1.0 -> 0.1.1
```

[semantic versioning standard]: https://semver.org/
