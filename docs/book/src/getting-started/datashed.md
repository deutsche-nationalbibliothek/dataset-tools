<p style="text-align: center;">
  <img  height="200" width="200" src="../img/datashed.svg" />
</p>

The `datashed` tool helps with the creation and maintenance of large
collections of text documents. The tool indexes the documents it
contains. Based on this index, data analyses can be carried out, for
example to identify corrupt documents or to build subsets based on text
statistical characteristics. Finally, the tool offers a HTTP/REST API to
use the index and the documents as a data source for building datasets.
The API can also be used to evaluate the quality of the documents or to
carry out further annotations.

# Tour

## Initialization

The [init] command is used to create a new datashed project. The
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
├── config.toml
└── tmp
```

The `data` directory contains the documents of the datashed. A document
must be in plain text format and end with the file extension `.txt`.
It is up to the user to set up an ingest process, e.g. in the form of a
Python script. Documents can be structured in any subdirectories within
the `data` directory.

## Configuration

The `config.toml` contains metadata about the project and important
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

Some configuration options can be set using the [config] command:

```console
$ datashed config runtime.normalization nfd
$ datashed config runtime.num-jobs 42
````

The `normalization` option specifies the Unicode normal form in which
the documents are encoded. Configuration options and command line
parameters are then transliterated into the corresponding normal form if
necessary. The `num-jobs` option defines the maximum number of CPU cores
used by the application.

## Indexing

The core of a datashed is the index of all documents. It contains
important metrics and metadata. The index is saved by default in the
file `index.ipc` in [Apache Arrow] format.

A new index can be created as follows:

```console
$ datashed index
Enumerating documents: 3 | elapsed: 00:00:00, done.
Indexing documents: 3 (100%) | elapsed: 00:00:00, done.
```

If the file name of the documents contains an identifier, this can be
written in an additional column using the `--filename-column` option.
In the following example, the index is extended with a new column `ppn`,
which contains the file name (without file extension):

```console
$ datashed index --filename-column "ppn"
Enumerating documents: 3 | elapsed: 00:00:00, done.
Indexing documents: 3 (100%) | elapsed: 00:00:00, done.
```

See [Datashed Index] for more information on the use and structure of
the index.


## Bibliographic References

With the help of the [bibrefs] command, bibliographic identifiers can be
found in documents. The following identifiers are supported:

| Reftype | Description                                                                                                           |
| ------- | --------------------------------------------------------------------------------------------------------------------- |
| `arxiv` | [arXiv Identifier (arXiv)](https://info.arxiv.org/help/arxiv_identifier.html)                                         |
| `ddc`   | [Dewey Decimal Classification (DDC)](https://en.wikipedia.org/wiki/Dewey_Decimal_Classification)                      |
| `doi`   | [Digital Object Identifier (DOI)](https://en.wikipedia.org/wiki/Digital_object_identifier)                            |
| `isbn`  | [International Standard Book Number (ISBN)](https://en.wikipedia.org/wiki/ISBN)                                       |
| `ismn`  | [International Standard Music Number (ISMN)](https://en.wikipedia.org/wiki/International_Standard_Music_Number)       |
| `isni`  | [International Standard Name Identifier (ISNI)](https://en.wikipedia.org/wiki/International_Standard_Name_Identifier) |
| `issn`  | [International Standard Serial Number (ISSN)](https://en.wikipedia.org/wiki/ISSN)                                     |
| `jel`   | [JEL Classification System (JEL)](https://www.aeaweb.org/econlit/jelCodes.php?view=jel)                               |
| `lcc`   | [Library of Congress Classification (LCC)](https://www.loc.gov/catdir/cpso/lcc.html)                                  |
| `msc`   | [Mathematics Subject Classification (MSC)](https://mathscinet.ams.org/mathscinet/msc/msc2020.html)                    |
| `orcid` | [Open Researcher and Contributor ID (ORCID)](https://en.wikipedia.org/wiki/ORCID)                                     |
| `udc`   | [Universal Decimal Classification (UDC)](https://en.wikipedia.org/wiki/Universal_Decimal_Classification)              |

The implementation is carried out using regular expressions, whereby the
expressions were formulated as strictly as possible in order to achieve
a very low error rate. If possible, the identifiers found are validated
(checksum verification).

For each match, the path (`path`), the hash (`hash`) and the location
within the document (`start` and `end`) are also recorded. The columns
`path` and `hash` can be used to JOIN with the index in order to get
additional metadata.

The `--normalize` flag can be used to convert identifiers into
a normalized, canonical form. When finding DOIs, the results can
be filtered by adding a path to a directory containing [Crossref]
(`--crossref` option) and/or [Datacite] (`--datacite` option) public
data files.

In the following example the list of bibliographic identifiers is
generated in normalized form using Crossref and Datacite allow lists:

```console
$ datashed bibrefs --normalize --datacite ~/datacite --crossref ~/crossref -o bibrefs.ipc
Processing Datacite: 5,930 (100%) | elapsed: 00:05:13, done.
Processing Crossref: 33,402 (100%) | elapsed: 00:05:54, done.
Processing documents: 10,439,309 (100%) | elapsed: 00:21:37, done.
```

The generated table looks like this:

```python
>>> import polars as pl
>>> bibrefs = pl.read_ipc("bibrefs.ipc", memory_map=False)
>>> bibrefs.sample(5)
shape: (5, 6)
┌───────────────────────┬──────────────┬─────────┬────────────────────────┬────────┬────────┐
│ path                  ┆ hash         ┆ reftype ┆ value                  ┆ start  ┆ end    │
│ ---                   ┆ ---          ┆ ---     ┆ ---                    ┆ ---    ┆ ---    │
│ str                   ┆ str          ┆ enum    ┆ str                    ┆ u64    ┆ u64    │
╞═══════════════════════╪══════════════╪═════════╪════════════════════════╪════════╪════════╡
│ ft/589/1265380589.txt ┆ 5152f83ab95d ┆ isbn    ┆ 978-3-8409-3021-8      ┆ 181920 ┆ 181938 │
│ ft/717/1292043717.txt ┆ e95bbc70fa79 ┆ doi     ┆ 10.1002/pamm.202200302 ┆ 58     ┆ 80     │
│ ft/013/1335636013.txt ┆ 839f31e41437 ┆ doi     ┆ 10.1002/jee.20201      ┆ 67929  ┆ 67946  │
│ ft/140/1127742140.txt ┆ c06d6d20641b ┆ ddc     ┆ 481.6                  ┆ 402168 ┆ 402173 │
│ ft/925/1256772925.txt ┆ 7affb2fb7c67 ┆ isbn    ┆ 978-3-8379-7673-1      ┆ 70964  ┆ 70982  │
└───────────────────────┴──────────────┴─────────┴────────────────────────┴────────┴────────┘
```


## Vocabulary

The [vocab] command can be used to create the vocabulary
\\(\mathcal{V}\\) (dictionary or lexicon) of the entire datashed or
of any subset. With the help of various filter options, the command
can be used to create stop word lists that are tailored to the entire
population. Also the (raw) features of subsets can be compared with
each other.

The output contains the terms with the corresponding _term frequency_
`tf` and the _document frequency_ `df`.

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

The option `--category` (`-L`) can be used to include only those terms
where at least on character belongs to the specified unicode category.
The following categories are available:

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
This function is provided by the [grep] command. It works in a similar
way to the Unix `grep` or `rg` command, but it supports other practical
functions.

Only the documents that have been indexed in datashed are searched.
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

The [lfreq] command generates a frequency table using a fixed
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


## Check

Invariants and constraints that the index (or the bibliographic
identifiers) must fulfill can be checked with the [check] command.
The command requires a configuration file containing a list of tests.
A test specification requires a unique ID and a query formulated in
SQL. The test fails if the SQL query does not evaluate to `true`.
Unless otherwise specified, the tests are automatically read from the
`checks.toml` file.

Optionally, a test can contain a description that is included in the
output. If a test is to be skipped, the `skip` flag can be set.

In the following example, the check verifies whether the file size is
the same for the same hash value (by shortening the hash, there is a
small chance that there will be a difference):

```toml
[check.I001]
description = "Same `hash` value implies same `size` value"
query = """
       SELECT COUNT(*) == 0
         FROM index AS lhs
   INNER JOIN index AS rhs
        USING (hash)
        WHERE lhs.size != rhs.size
"""
```

The command is invoked as follows:

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

If at least on check fails, the command returns a non-zero status
code. This made it possible for the quality checks to be carried out
automatically in a data pipeline. The following example demonstrates the
integration as a separate stage in a [DVC] pipeline:

```yaml
stages:
  check:
    cmd:
      - datashed check
    deps:
      - checks.toml
      - index.ipc
```

## Summary Statistics

The [summary] command can be used to create a summary statistics of a
datashed. The output is in JSON format.

```console
$ datashed summary | jq .
{
  "docs": 3,
  "size": 3138
}
```

The command is suitable for being integrated into a [DVC] pipeline. This
makes it possible to compare the change in inventory among different
ingest runs.

```yaml
stages:
  summary:
    cmd:
      - datashed summary -o summary.json
    metrics:
      - summary.json
```

## Data Verification

The [verify] command is used to ensure that the index and the documents
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

<!--

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
-->

[Apache Arrow]: https://arrow.apache.org
[Crossref]: https://www.crossref.org/learning/public-data-file
[Datacite]: https://datafiles.datacite.org
[Datashed Index]: ../concepts/datashed-index.md
[DVC]: https://dvc.org/

[bibrefs]: ../reference/datashed/commands/datashed-bibrefs.md
[check]: ../reference/datashed/commands/datashed-check.md
[config]: ../reference/datashed/commands/datashed-config.md
[grep]: ../reference/datashed/commands/datashed-grep.md
[init]: ../reference/datashed/commands/datashed-init.md
[lfreq]: ../reference/datashed/commands/datashed-lfreq.md
[summary]: ../reference/datashed/commands/datashed-summary.md
[verify]: ../reference/datashed/commands/datashed-verify.md
[vocab]: ../reference/datashed/commands/datashed-vocab.md
