# Dataset Tools

This project contains tools for data preparation, transformation and
analysis to create text corpora that can be processed with [Annif]. The
tools are based on experiments and prototypes that were developed as
part of the research work in the _Automated Indexing System_ project of
the [German National Library].

## Tools

The [datashed] tool helps with the creation and maintenance of large
collections of text documents. The tool indexes the documents it
contains. Based on this index, data analyses can be carried out, for
example to identify corrupt documents or to build subsets based on text
statistical characteristics. Finally, the tool offers a HTTP/REST API to
use the index and the documents as a data source for building datasets.
The API can also be used to evaluate the quality of the documents or to
carry out further annotations.



[Annif]: https://annif.org
[datashed]: ./getting-started/datashed.md
[German National Library]: https://www.dnb.de
