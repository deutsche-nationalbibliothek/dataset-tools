# datashed-vocab(1)

## NAME

*datashed-vocab* --- Create vocabulary (set of terms) statistics

## SYNOPSIS

`datashed vocab` [_OPTIONS_]

## DESCRIPTION

The `vocab` command can be used to create the vocabulary (dictionary
or lexicon) of the entire datashed or of any subset. With the help of
various filter options, the command

## OPTIONS

`-b`, `--bigrams`
: Use two adjacent words as vocabulary terms

`-t`, `--trigrams`
: Use three adjacent words as vocabulary terms

`-S`, `--stopwords <filename>`
: Exclude words that are contained in the stop word list

`-L`, `--category <category>`
: Includes only those terms in the vocabulary where at least one
character belongs to one of the specified unicode categories. Possible
categories: all (`a`), lowercase (`l`), uppercase (`u`), titlecase
(`t`), modifier (`m`), or other (`o`).

`--min-term-length <n>`
: Ignore tokens with a length less than `<n>` (default: 2).

`--min-term-freq <n>`
: Ignore tokens with a term frequency less than `<n>`.

`--min-doc-freq <n>`
: Ignore tokens with a document frequency less than `<n>`.

`-l`, `--limit <n>`
: Limits the output to the n most frequent tokens

`-o`, `--output <filename>`
: Write the result to `<filename>`. By default output will be written in
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

```console
$ datashed vocab -q --bigrams -S ~/stopwords.txt \
    --where "doctype == 'scientific-article'" -o vocab.ipc
```

