# Dataset Tools

This project contains tools for data preparation, transformation and
analysis to create text corpora that can be processed with [Annif]. The
tools are based on experiments and prototypes that were developed as
part of the research work in the _Automated Indexing System_ project of
the [German National Library] (DNB). This project is developed by the
Metadata Department of the DNB.

## Tools

### Datashed

The [datashed] tool helps with the creation and maintenance of large
collections of text documents. The tool indexes the documents it
contains. Based on this index, data analyses can be carried out, for
example to identify corrupt documents or to build subsets based on text
statistical characteristics. Finally, the tool offers a HTTP/REST API to
use the index and the documents as a data source for building datasets.
The API can also be used to evaluate the quality of the documents or to
carry out further annotations.

### Dataset

> [!WARNING]
> This tool is still incomplete and only a prototype.

The [dataset] tool creates [Annif]-compatible corpora from the documents
in one or more datasheds.

## License

The source code is licensed under the [European Union Public License 1.2].

[Annif]: https://annif.org
[dataset]: ./getting-started/dataset.md
[datashed]: ./getting-started/datashed.md
[German National Library]: https://www.dnb.de
[European Union Public License 1.2]: https://github.com/deutsche-nationalbibliothek/dataset-tools/blob/main/LICENSE
