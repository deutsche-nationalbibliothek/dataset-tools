# datashed-check(1)

## NAME

*datashed-check* --- Executes checks to ensure the integrity of the
index

## SYNOPSIS

`datashed check` [_OPTIONS_] [_CONFIG_]

## DESCRIPTION

This command executes checks to ensure the integrity of the index.

## ARGUMENTS

`CONFIG`
: The configuration file that contains the checks to be performed
(default `config.toml`).

## OPTIONS

`-B`, `--bibrefs <filename>`
: Specify a file containing bibliographic identifiers so that these can
also be checked.

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
$ datashed check
        PASS [    0.021s] I001 ⊢ Expect all documents to be non-empty (`size` > 0)
        PASS [    0.416s] I002 ⊢ The `ppn` column must contain valid PPNs
        PASS [   18.589s] I003 ⊢ Same `hash` value implies same `size` value
        PASS [   19.492s] I004 ⊢ The `path` starting with 'toc/' implies `doctype` 'toc'
        PASS [   19.539s] I005 ⊢ The `lang.score` value is taken from the interval [0, 1]
        PASS [   19.590s] I006 ⊢ A `alpha` value is taken from the interval [0, 1]
────────────
     Summary 6 checks executed: 6 passed, 0 skipped, 0 failed
```
