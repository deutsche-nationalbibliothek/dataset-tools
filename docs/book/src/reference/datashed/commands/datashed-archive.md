# datashed-archive(1)

## NAME

*datashed-archive* --- Create an archive of the index, config and all
documents.

## SYNOPSIS

`datashed archive` [_OPTIONS_]

## DESCRIPTION

This command create an archive of the index, config and all documents.

## OPTIONS

`--fast`
: Uses the lowest compression at the highest speed

`--best`
: Uses the best compression at the lowest speed

`-o`, `--output <filename>`
: Write the archive to `filename` instead of stdout

`-h`, `--help`
: Print help

### COMMON OPTIONS

{{ #include common-opts.md }}

## EXIT STATUS

{{ #include exit-status.md }}

## EXAMPLES

```console
$ datashed archive -o ~/tmp/backup.tar.gz
Archive documents: 3 (100%) | elapsed: 00:00:00, done.
```
