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
$ dataset remote add arxiv-data https://example.com:1234
$ dataset remote add wikipedia-data https://example.com:2345
```

Optionally, you can specify a filter expression to be applied to a
datashed's index. For example, the following command will only include
documents from the `arxiv-data` datashed that have a file size greater
than 1 KiB:

```console
$ dataset remote add arxiv-data https://example.com:1234 --where "size > 1024"
```


[Annif]: https://annif.org

[init]: ../reference/dataset/commands/dataset-init.md
[remote]: ../reference/dataset/commands/dataset-remote.md
