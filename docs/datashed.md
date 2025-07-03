<p align="center">
  <img  height="250" width="250" src="../.github/datashed.svg" />
</p>

<hr />

The `datashed` tool is a _reverse ETL_ program that indexes the
documents it contains. Based on this index, data analyses can be carried
out, for example to identify corrupt documents or to build subsets based
on text statistical characteristics. Finally, the tool offers a GraphQL
API to use the index and the documents as a data source for building
datasets. The API can also be used to evaluate the quality of the
documents or to carry out further annotations.

# Tour

## Initialization

The `init` command is used to create a new datashed project. The
following command creates a new project `demo`.

```console
$ datashed init demo
```

Alternatively, a project can be initialized inside an existing
directory:

```console
$ mkdir demo && cd demo
$ datashed init .
```

### Project structure

An empty project consists of the following files and directories:

```console
$ tree demo
├── data
├── datashed.toml
└── tmp
```

The `data` directory contains the documents of the datashed. A document
must be in plain text format and end with the file extension `.txt`.
It is up to the user to set up an ingest process, e.g. in the form of a
Python script. Documents can be structured in any subdirectories within
the `data` directory.

### Configuration

The `datashed.toml` contains metadata about the project and important
runtime options.

```toml
[metadata]
name = "demo"
version = "0.1.0"
authors = ["Jane Doe <jane.doe@example.com"]  
```

The metadata can be changed by command line options (see `datashed init
--help`). By default, the name of the project is the directory name,
the initial version is `0.1.0` and the author is derived from the Git
identity (if possible). The project is also automatically initialized
as a Git repository. This behavior can be deactivated using the `--vcs
none` option.

Some configuration options can be set using the `config` command:

```console
$ datashed config runtime.normalization nfd
$ datashed config runtime.num-jobs 42
````

The `normalization` option specifies the Unicode normal form in which
the data is encoded. Configuration options and command line parameters
are then transliterated into the corresponding normal form if necessary.
The `num-jobs` option defines the maximum number of CPU cores used by
the application.


## Indexing

The core of a datashed is the index of all documents. It contains
important metrics and metadata. The index is saved by default in the
file `index.ipc` in [Apache Arrow] format.

A new index can be created as follows:

```console
$ datashed index
Collecting documents: 3 | elapsed: 00:00:00, done.
Indexing documents: 3 (100%) | elapsed: 00:00:00, done.
```

If the file name of the documents contains an identifier, this can be
written in an additional column using the `--filename-column` option.
In the following example, the index is extended with a new column `ppn`,
which contains the file name (without file extension):

```console
$ datashed index --filename-column "ppn"
Collecting documents: 3 | elapsed: 00:00:00, done.
Indexing documents: 3 (100%) | elapsed: 00:00:00, done.
```

### Metadata and Metrics

#### Hash

The `hash` column contains a six-byte hexadecimal hash of the document.
The hash value is derived from the document's SHA256 checksum, by taking
the first six bytes and format them as a hexadecimal string. Documents
that have the same hash value are very likely to have the same content
and should be treated as duplicates.

#### Language

The language of the document is given as an ISO-639-2 (B) language
code in the `lang_code` column. The `lang_score` column contains the
confidence value. If the index is in [Apache Arrow] format, which is the
default, the two columns are combined into one column `lang`. Language
detection is performed with [lingua] in high-accuracy mode.


#### Alpha

The `alpha` score of a document is the ratio of alphabetic characters to
the total number of characters. An alphabetic character is a character
which satisfy the _Alphabetic_ property of the [Unicode Standard]
described in Chapter 4 (Character Properties). The score is defined as

$$
alpha \triangleq \frac{1}{N}\sum_{i = 1}^{N} \mathbf{1}_A(c_i)
$$

where $N$ is total number of characters of the document, $c_i$ is the
i-th character of the document, $A$ is the subset of all characters,
which satisfy the _Alphabetic_ property and $\mathbf{1}_A$ is the
indicator function, which returns 1 if the i-th character is alphabetic
and otherwise 0. The range of the function is $[0, 1]$ and the score of
an empty  document is defined to $0.0$.

A very low `alpha` value may indicate a corrupt document.


## Vocabulary

The `vocab` command can be used to create the vocabulary (dictionary
or lexicon) of the entire datashed or of any subset. With the help
of various filter options, the command can be used to create stop
word lists that are tailored to the entire population. Also the (raw)
features of subsets can be compared with each other.

The output contains the terms with the corresponding term frequency
(`tf`) and the document frequency (`df`).

In the following example, the vocabulary of all documents in the datashed
is created:

```console
$ datashed vocab -q -o vocab.csv
$ head -4 vocab.csv
term,tf,df
die,16,2
und,16,2
deutsche,8,3
```

To use two or three adjacent words as vocabulary terms, the options
`--bigrams` / `-b` or `--trigrams` / `-t` can be used:

```console
$ datashed vocab -q --bigrams | head -3
term,tf,df
die tib,5,1
die deutsche,4,2
```

Using the `--stopwords` / `-S` option, all terms that appear in a
stopword list can be removed in advance:

```console
$ echo "die\nund\n" >> STOPWORDS.txt
$ datashed vocab -q -S STOPWORDS.txt | head -4
term,tf,df
deutsche,8,3
in,8,3
the,8,1
```

The option `--category` (`-L`) can be used to include only those
terms where at least on character belongs to the specified unicode
category. The following categories are available:

* `a` (`all`) —  "Letter" category _Lc_, _Ll_, _Lm_, _Lo_, _Lt_, _Lu_,
* `l` (`lowercase`) — "Letter, Lowercase" category _Ll_,
* `u` (`uppercase`) — "Letter, Uppercase" category _Lu_,
* `t` (`titlecase`) — "Letter, Titlecase" category _Lt_,
* `m` (`modifier`) — "Letter, Modifier" category _Lm_,
* `o` (`other`) — "Letter, Other" category _Lo_.

In addition, the vocabulary can be further restricted by the
`--min-term-length`, `--min-term-freq`, or `--min-doc-freq` options.


## Grepping

A simple form of document retrieval is a linear search for patterns.
This function is provided by the `grep` command. It works in a similar
way to the Unix `grep` or `rg` command, but it supports other practical
functions.

Only the documents that have been indexed in Datashed are searched.
The output contains all lines from the index where the corresponding
document matches one of the specified patterns. By default, the output
is written in CSV format on the console.

In the following example, the documents are searched for the phrase
_"(DNB)"_:

```console
$ datashed grep -q '\(DNB\)'
path,hash,size,mtime
0/dnb.txt,71eb6431,769,1750321974
```

The index can be restricted in advance according to conditions. If, for
example, only documents with a file size of less than 1 KiB are to be
searched, this is done using the `--where` option:

```console
$ datashed grep -q '\(DNB\)' --where 'size <= 1024'
path,hash,size,mtime
0/dnb.txt,71eb6431,769,1750321974
```

Another useful option is the restriction to a search window. If only
the first _n_ bytes are to be searched, this is done by specifying the
`-n` option:

```console
$ datashed grep -q -n 50 '\(DNB\)'
path,hash,size,mtime
0/dnb.txt,71eb6431,769,1750321974
```

## Letter Frequency

The `lfreq` command generates a frequency table using a fixed
alphabet. Both the specified alphabet and the documents themselves
are transliterated in advance into the Uniode normal form NFC and
converted into lower case letters. All characters that are not part
of the alphabet are ignored. Unless a alphabet is specified using the
`--alphabet` option, the German alphabet `a` to `z`, `ä`, `ö`, `ü`, `ß`
is used by default.

For each document, the output contains the path (`path`), the hash value
of the document (`hash`), the total number of characters (`total`) and
the individual letter frequencies generated.

In the following example, a frequency table is created using the
alphabet `a`, `b` and `c`:


```console
$ datashed lfreq -q --alphabet "abc"
path,hash,total,a,b,c
0/dnb.txt,1fbf52b4,85,38,26,21
0/tib.txt,809239e5,135,56,35,44
1/zbw.txt,a50f7e55,118,59,19,40
```

## Summary Statistics

The `summary` command can be used to create a summary statistics of
a datashed. The output is in JSON format.

```console
$ datashed summary | jq .
{
  "docs": 3,
  "size": 3138
}
```

The command is suitable for being integrated into a DVC pipeline. This
makes it possible to compare the change in inventory among different
ingest runs.

```json
stages:
  summary:
    cmd:
      - datashed summary -o summary.json
    metrics:
      - summary.json
```

## Data Verification

The `verify` command is used to ensure that the index and the documents
are in a consistent state. It ensures that no documents have been
deleted and that the contents of the documents have not changed. You can
choose between two modes: In `permissive` mode, the SHA256 checksums of
the documents must match. If the `strict` mode is selected, the system
also checks that the date of the last modification has not changed. By
default, the command uses the `strict` mode. If the `verify` command
fails, an error message is printed to `stderr` and a non-zero status
code is returned.

```console
$ datashed verify --mode permissive
Verifying documents: 3 (100%) | elapsed: 00:00:00, done.

$ datashed verify --mode strict
Verifying documents: 3 (100%) | elapsed: 00:00:00, done.
```

## Versioning

It is good practice to track changes to a project's database with
version numbers. Using the `version` command, the version of the project
can either be changed or incremented. The version must follow the
[Semantic Versioning](https://semver.org/) guidelines.

The current version of the project can be queried as follows:

```console
$ datashed version
0.1.0
```

The following command changes the version of the project to the value
`0.2.0`. Note, that unless the `-f` (`--force`) flag is set, the new
version must always be greater than the current version.

```console
$ datashed version 0.2.0
```

It is also possible to increment only the _major_, _minor_ or _patch_
version:

```console
$ datashed version --bump major
$ datashed version --bump minor
$ datashed version --bump patch
```

## Archive and Restore

The `archive` command can be used to create a backup of a datashed. It
creates a `tar.gz` archive containing all documents, the configuration
and the current index. It is important to note, that only the documents
contained in the index are archived. If there are documents that have
not yet been indexed, the index should be updated first. By default,
the compression is biased towards high compression ratio at expense of
speed. This behavior can be changed using the `--fast` or `--best` flag.

```console
$ datashed archive -o ~/tmp/backup.tar.gz
Archive documents: 3 (100%) | elapsed: 00:00:00, done.
```

An archive can be restored either via the `tar` program or by using
the `restore` command. By default, the archive is restored inside the
current directory. If the archive is to be unpacked into a different
directory, this can be specified with the `-C` (`--directory`) option.
The new  directory is created automatically if it does not yet exist.

```console
$ datashed restore ~/tmp/backup.tar.gz -C foobar
Successfully restored archive.
Verify consistency with `datashed verify`.
```


[Apache Arrow]: https://arrow.apache.org/
[Unicode Standard]: https://www.unicode.org/versions/latest/
[lingua]: https://github.com/pemistahl/lingua-rs
