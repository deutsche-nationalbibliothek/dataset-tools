# datashed-grep(1)

## NAME

*datashed-grep* --- Find documents matching patterns

## SYNOPSIS

`datashed grep` [_OPTIONS_] [_PATTERN_]...

## DESCRIPTION

This command find documents matching the given regular expressions.

## ARGUMENTS

`<PATTERN>`
: Regular expression used for searching

## OPTIONS

`-n`, `--max-bytes <n>`
: Use only the first n bytes to search for the given pattern. If the
value is 0 or greater than the document size, the entire document is
used for searching.

`-i`, `--ignore-case`
: If set, all patterns will be search case insensitive.

`--invert-match`
: Keep documents that don't match

`-o`, `--output <filename>`
: Write the result to  `filename`. By default output will be written in
CSV format to `stdout`.

`-h`, `--help`
: Print help

### COMMON OPTIONS

{{ #include common-opts.md }}

## EXIT STATUS

{{ #include exit-status.md }}

## EXAMPLES

```console
$ datashed grep -q '\(DNB\)' -o grep.csv
```
