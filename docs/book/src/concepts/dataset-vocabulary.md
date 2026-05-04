# Dataset Vocabulary

If a vocabulary is required in the dataset, it can be defined in the
vocabulary section. There are three different methods available for
selecting and defining the vocabulary: specifying a fixed list of
concepts, or dynamically generating it from a PICA+ or MARC21 dump.

The vocabulary is generated in [RDF/SKOS] format and is saved by default
to the file `vocab.ttl`.

## Fixed Listing

In its simplest form, the vocabulary is generated from a fixed list
of concepts. These are defined in the `[vocabulary]` section of
`params.toml` using the following configuration options:

`output`
: Specify the file to which the output will be written. If this option
is not specified, the vocabulary will automatically be written to the
file `vocab.ttl`. Note: Currently, only the TTL format is supported.

`base-uri`
: Specification of the base URI from which the concept's URI is formed
by concatenating it with the concept's notation.

`concepts`
: The list of vocabulary items.

A concept is always defined by specifying the notation corresponding
to the form used in the gold standard and a list of labels. A label
always consists of the `kind` (`preferred`, `alternative`, `hidden`),
the `label` itself, and a language tag (`lang`). If the type is not
specified, a preferred label is assumed by default.

### Examples

```toml
[vocabulary]
base-uri = "https://d-nb.info/standards/classification/ddc-sc#"
concepts = [
  {
    notation = "000",
    labels = [
      { label = "Allgemeines, Wissenschaft", lang = "de" },
      { label = "Generalities, science", lang = "en" },
    ]
  },
  {
    notation = "004",
    labels = [
      { label = "Informatik", lang="de" },
      { label = "Computer science", lang="en" },
     ]
  },
]
```

```xml
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

<https://d-nb.info/standards/classification/ddc-sc#000> a <http://www.w3.org/2004/02/skos/core#Concept>;
  <http://www.w3.org/2004/02/skos/core#prefLabel> "Allgemeines, Wissenschaft"@de,
    "Generalities, science"@en.

<https://d-nb.info/standards/classification/ddc-sc#004> a <http://www.w3.org/2004/02/skos/core#Concept>;
  <http://www.w3.org/2004/02/skos/core#prefLabel> "Informatik"@de,
    "Computer science"@en.
```
 
## MARC21

> [!NOTE]
> This feature is not yet available and is still under development.

## PICA+

> [!NOTE]
> This feature is not yet available and is still under development.


[RDF/SKOS]: https://www.w3.org/TR/skos-primer/
