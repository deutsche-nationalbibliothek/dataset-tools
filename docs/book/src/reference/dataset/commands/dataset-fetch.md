# dataset-fetch(1)

## NAME

*dataset-fetch* --- Download indices and metadata from datasheds

## SYNOPSIS

`dataset fetch` [_OPTIONS_]

## DESCRIPTION

Once the remotes have been added, the indices can be downloaded using
the `fetch` command. If a remote has a pre-filter, it will be applied to
the index. The individual indices are stored in the `.dataset/remotes`
directory and form the population from which corpora can be created.

## OPTIONS

`--dry-run`
: Show index size, without making any changes.

`-h`, `--help`
: Print help

### COMMON OPTIONS

{{ #include common-opts.md }}

## EXIT STATUS

{{ #include exit-status.md }}

## EXAMPLES

The following command fetches the indices from the remotes `arxiv-data`
and `wp-data`:

```console
$ dataset fetch
arxiv-data: 2,011,265 documents (108.17 GiB, delta 2011265), done.
wp-data: 665,915 documents (4.99 GiB, delta 665915), done.
```
