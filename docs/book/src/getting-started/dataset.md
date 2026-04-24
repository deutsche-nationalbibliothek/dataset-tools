# Dataset

The `dataset` tool creates [Annif]-compatible corpora from the documents
in one or more datasheds.

# Tour

## Initialization

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

### Project structure

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


[Annif]: https://annif.org

[init]: ../reference/dataset/commands/dataset-init.md
