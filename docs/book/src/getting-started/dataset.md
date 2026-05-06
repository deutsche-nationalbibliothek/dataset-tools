# Dataset

The `dataset` tool creates [Annif]-compatible corpora from the documents
in one or more datasheds.

## Tour

### Initialization

A new dataset project can be created using the [init] command. The
following command creates a new dataset `demo`:

```console
$ dataset init demo
```

Alternatively, a dataset can be initialized inside an existing
directory:

```console
$ mkdir demo && cd demo
$ dataset init .
```

#### Project structure

An empty dataset consists of the following files and directories:

```console
$ tree -a            
.
├── .dataset
│   └── config.toml
└── .gitignore
```

The `.dataset` directory contains the configuration file as well as
artifacts and metadata required for managing the data.

### Remotes

To use the documents from a datashed, you must add them to the project
using their URL via the [remote] command:

```console
$ dataset remote add arxiv-data https://example.com:9500
$ dataset remote add wp-data https://example.com:9400
```

Optionally, you can specify a pre-filter expression to be applied to a
datashed's index. For example, the following command will only include
documents from the `arxiv-data` datashed that have a file size greater
than 1 KiB:

```console
$ dataset remote add arxiv-data https://example.com:9500 --where "size > 1024"
```

### Fetch

Once the remotes have been added, the indices can be downloaded using
the [fetch] command. If a remote has a pre-filter, it will be applied to
the index. These indices form the population from which corpora can
be created.

```console
$ dataset fetch
arxiv-data: 2,011,265 documents (108.17 GiB, delta 2011265), done.
wp-data: 665,915 documents (4.99 GiB, delta 665915), done.
```

The individual indices are stored in the `.dataset/remotes` directory:

```console
$ tree --noreport .dataset/remotes
.dataset/remotes
├── arxiv-data.ipc
└── wp-data.ipc
```

The `delta` value indicates the difference from a previously retrieved
index. If no index has been retrieved yet, the value corresponds to
the size of the initial index. To simply check whether a remote has new
documents, you can use the `--dry-run` option:

```console
$ dataset fetch --dry-run
arxiv-data: 2,011,265 documents (108.17 GiB, delta 0), done.
wp-data: 665,915 documents (4.99 GiB, delta 0), done.
```

### Create & Update

#### Vocabulary

If a vocabulary is required in the dataset, it can be defined in the
`[vocabulary]` section of the `params.toml` (see [Dataset Vocabulary]
for more information). The following example creates a vocabulary from a
fixed list and saves it in the file `vocab.ttl` ([RDF/SKOS] format):

```toml
[vocabulary]
concepts = [
  {
    notation = "000",
    labels = [
      { kind = "preferred", label = "Allgemeines, Wissenschaft", lang = "de" },
      { kind = "preferred", label = "Generalities, science", lang = "en" },
    ]
  },
  ...
]
```

[Annif]: https://annif.org
[Dataset Vocabulary]: ../concepts/dataset-vocabulary.md
[RDF/SKOS]: https://www.w3.org/TR/skos-primer/

[init]: ../reference/dataset/commands/dataset-init.md
[fetch]: ../reference/dataset/commands/dataset-fetch.md
[remote]: ../reference/dataset/commands/dataset-remote.md

