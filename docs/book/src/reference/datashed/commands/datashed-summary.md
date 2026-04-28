# datashed-summary(1)

## NAME

*datashed-summary* --- Creates summary statistics in JSON format

## SYNOPSIS

`datashed summary` [_OPTIONS_]

## DESCRIPTION

This command creates summary statistics in JSON format.

## OPTIONS

`-o`, `--output <OUTPUT>`
: Write the result to `<OUTPUT>`. By default output will be written in
JSON format to `stdout`.

`-h`, `--help`
: Print help

### FILTER OPTIONS

{{ #include filter-opts.md }}

### COMMON OPTIONS

{{ #include common-opts.md }}

## EXIT STATUS

{{ #include exit-status.md }}

## EXAMPLES

```console
$ datashed summary | jq .
{
  "docs": 3,
  "size": 3138
}
```
