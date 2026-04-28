# datashed-bibrefs(1)

## NAME

*datashed-bibrefs* --- Extract bibliographic identifiers from documents

## SYNOPSIS

`datashed bibrefs` [_OPTIONS_]

## DESCRIPTION

This command extracts bibliographic identifiers from documents.

## OPTIONS

`--normalize`
: Whether to normalize bibliographic references or not.

`--crossref <CROSSREF>`
: Use the [Crossref Public Data File] as a source for valid DOIs.

`--datacite <DATACITE>`
: Use the [DataCite Public Data File] as a source for valid DOIs.

`-o`, `--output <OUTPUT>`
: Write the result to `<OUTPUT>`. By default output will be written in
CSV format to `stdout`.

`-h`, `--help`
: Print help

### FILTER OPTIONS

{{ #include filter-opts.md }}

### COMMON OPTIONS

{{ #include common-opts.md }}

## EXIT STATUS

{{ #include exit-status.md }}

## EXAMPLES

The following command extracts bibliographic identifiers using a
specified filter. The results are normalized.

```console
$ datashed bibrefs -q --where "doctype == 'scientific-article'" -o bibrefs.ipc
```

[Crossref Public Data File]: https://www.crossref.org/learning/public-data-file
[DataCite Public Data File]: https://support.datacite.org/docs/datacite-public-data-file
