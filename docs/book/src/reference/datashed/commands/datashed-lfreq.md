# datashed-lfreq(1)

## NAME

*datashed-lfreq* --- Create a frequency table over a fixed alphabet

## SYNOPSIS

`datashed lfreq` [_OPTIONS_]

## DESCRIPTION

This command creates frequency table over a fixed alphabet.

## OPTIONS

`--alphabet <alphabet>`
: The alphabet used to determine the letter frequencies (default: German
alphabet).

`-o`, `--output <filename>`
: Write output to `filename` instead of `stdout`.

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
$ datashed lfreq -q --alphabet "abc"
path,hash,total,a,b,c
0/dnb.txt,1fbf52b4,85,38,26,21
0/tib.txt,809239e5,135,56,35,44
1/zbw.txt,a50f7e55,118,59,19,40
```
