# Datashed Refinements

In order to derive the document type from metadata, a PICA+ dump
can optionally be specified. The rule file must be specified in the
`refinements` section:

```toml
[refinements]
doctype = "doctype.toml"
```

A refinement can either be formulated as an `if-then` or as a `match`
expression:

```toml
[[refinements]]
match = '037C.d'
cases = [
  { pattern = 'Bachelorarbeit', then = 'bachelor-thesis' },
  { pattern = 'Dissertation', then = 'doctoral-thesis' },
  { pattern = 'Masterarbeit', then = 'master-thesis' },
]

[[refinements]]
if = '017A.a == "nt"'
then = 'musical-notation'
```

The document type is derived from the metadata if a corresponding PICA+
dump is specified as a comma line option:

```console
$ datashed index --filename-column "ppn" DUMP.dat.gz
Processing metadata: 100 | elapsed: 00:00:00, done.
Enumerating documents: 3 | elapsed: 00:00:00, done.
Indexing documents: 3 (100%) | elapsed: 00:00:00, done.
```
