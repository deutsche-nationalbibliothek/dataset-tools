# dataset-remote(1)

## NAME

*dataset-remote* --- Manage set of tracked datasheds (data sources)

## SYNOPSIS

`dataset remote add` _\<NAME\>_ _\<URL\>_ [`--where` _\<PREDICATE\>_]\
`dataset remote remove ` _\<NAME\>_\
`dataset remote set-predicate` _\<NAME\>_ _\<PREDICATE\>_\
`dataset remote set-url` _\<NAME\>_ _\<URL\>_

## DESCRIPTION

This command is used to manage the various datasheds (data sources).

## ARGUMENTS

`<NAME>`
: The name of the remote.

`<URL>`
: The url of the remote. Currently, only the url scheme HTTPS is
supported.

`<PREDICATE>`
: The predicate (filter criterion) applied to the datashed index.

## OPTIONS

`-h`, `--help`
: Print help

### COMMON OPTIONS

{{ #include common-opts.md }}


## EXIT STATUS

{{ #include exit-status.md }}

## EXAMPLES

The following command add two remotes to the list of tracked datasheds:

```console
$ dataset remote add arxiv-data https://example.com:1234 --where "size > 1024"
$ dataset remote add wp-data https://example.com:2345--where "size > 1024"
```
