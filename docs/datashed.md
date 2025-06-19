# datashed

## Tour

### Archive

The `archive` command can be used to create a backup of a fileshed. It
creates a tar.gz archive containing all documents, the configuration
and the current index. Only the documents contained in the index are
archived.

```console
$ datashed archive -o ~/tmp/backup.tar.gz
Archive documents: 3 (100%) | elapsed: 00:00:00, done.
```

