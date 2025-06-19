# datashed

## Tour

### Archive

The `archive` command can be used to create a backup of a datashed. It
creates a `tar.gz` archive containing all documents, the configuration
and the current index.

> [!IMPORTANT]
> Only the documents contained in the index are archived. If there are
> documents that have not yet been indexed, the index should be updated
> before archiving.

```console
$ datashed archive -o ~/tmp/backup.tar.gz
Archive documents: 3 (100%) | elapsed: 00:00:00, done.
```

