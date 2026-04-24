# datashed-config(1)

## NAME

*datashed-config* --- Get and set datashed config options

## SYNOPSIS

`datashed config` [_OPTIONS_] _\<NAME\>_ [_VALUE_]

## DESCRIPTION

This command allows to get and set config options of a datashed.

## ARGUMENTS

`<NAME>`
: The name of the config option.

`VALUE`
: The (new) value of the config option

## OPTIONS

`--get`
: Get the value for the given key

`--unset`
: Remove the key from the config

`--set`
: Set the value for the given key

`-h`, `--help`
: Print help

### COMMON OPTIONS

{{ #include common-opts.md }}

## EXIT STATUS

{{ #include exit-status.md }}

## EXAMPLES

The following command prints the current value of the config option
`runtime.num-jobs`:

```console
$ datashed config runtime.num-jobs
runtime.num-jobs = 10
```

The value can be changed as follow:

```console
$ datashed config runtime.num-jobs 23
$ datashed config runtime.num-jobs
runtime.num-jobs = 23
```

To reset the value to its default, use the `--unset` option:

```console
$ datashed config --unset runtime.num-jobs
$ datashed config runtime.num-jobs
runtime.num-jobs = None
```
