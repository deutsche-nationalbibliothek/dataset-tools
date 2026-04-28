# datashed-restore(1)

## NAME

*datashed-restore* --- Restore a datashed archive

## SYNOPSIS

`datashed restore` [_OPTIONS_] _\<ARCHIVE\>_

## DESCRIPTION

This command restores a datashed archive.

## OPTIONS

`-C`, `--directory <directory>`
: The destination directory (default: `.`).

`-h`, `--help`
: Print help

### COMMON OPTIONS

{{ #include common-opts.md }}

## EXIT STATUS

{{ #include exit-status.md }}

## EXAMPLES

```console
$ datashed restore ~/tmp/backup.tar.gz -C foo
Successfully restored archive.
Verify consistency with `datashed verify`.
```
