# datashed-index(1)

## NAME

*datashed-index* --- Create an index of all available documents

## SYNOPSIS

`datashed index` [_OPTIONS_] [_\<METADATA\>_]

## DESCRIPTION

This command indexes all available documents of the datashed.

## ARGUMENTS

`<METADATA>`
: Optionally, the index can be enriched with the `genre`, `group`, and
`doctype` columns using a metadata extract (PICA+ format). For more
information, see [Refinements].

## OPTIONS

`--filename-column <COLUMN>`
: Write the filename (without extension) into the specified column

`--with-doctype`
: Whether to add a doctype column or not

`--doctype <DOCTYPE>`
: The default document type if the metadata could not be used to
determine the document's type (default: `none`).

`--with-genre`
: Whether to add a `genre` column or not

`--genre <GENRE>`
: The default genre if the metadata could not be used to
determine the document's genre (default: `none`).

`--with-group`
: Whether to add a `group` column or not

`--group <GROUP>`
: The default group if the metadata could not be used to
determine the document' group (default: `none`).

`-l`, `--limit` `<N>`
: Stop processing after _N_ documents (default: 0).

`-o`, `--output` `<OUTPUT>`
: Write the index to <OUTPUT> instead to `index.ipc`.


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

[Refinements]: ../../../concepts/datashed-refinements.md
