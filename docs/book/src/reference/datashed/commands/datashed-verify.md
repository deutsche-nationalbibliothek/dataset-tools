# datashed-verify(1)

## NAME

*datashed-verify* --- Verify whether the metadata conforms to the
inventory

## SYNOPSIS

`datashed verify` [_OPTIONS_]

## DESCRIPTION

The `verify` command is used to ensure that the index and the documents
are in a consistent state. It ensures that no documents have been
deleted and that the contents of the documents have not changed. You can
choose between two modes: In `permissive` mode, the SHA256 checksums of
the documents must match. If the `strict` mode is selected, the system
also checks that the date of the last modification has not changed. By
default, the command uses the `strict` mode. If the `verify` command
fails, an error message is printed to `stderr` and a non-zero status
code is returned.


## OPTIONS

`-m`, `--mode <mode>`
: Set the verify mode (default: `strict`). Possible modes: `permissive`,
`strict`.

`-h`, `--help`
: Print help

### COMMON OPTIONS

{{ #include common-opts.md }}

## EXIT STATUS

{{ #include exit-status.md }}

## EXAMPLES

```console
$ datashed verify -q --mode permissive
$ datashed verify -q --mode strict
```
