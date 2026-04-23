# dataset-tools

[![CI](https://github.com/deutsche-nationalbibliothek/dataset-tools/actions/workflows/ci.yaml/badge.svg)](https://github.com/deutsche-nationalbibliothek/dataset-tools/actions/workflows/ci.yaml)

This project contains tools for data preparation, transformation and
analysis to create text corpora that can be processed with [Annif]. The
tools are based on experiments and prototypes that were developed as
part of the research work in the _Automated Indexing System_ project of
the [German National Library].


## Tools

The [datashed] tool is a _reverse ETL_ program that indexes the
documents it contains. Based on this index, data analyses can be carried
out, for example to identify corrupt documents or to build subsets
based on text statistical characteristics. Finally, the tool offers a
HTTP/REST API to use the index and the documents as a data source for
building datasets. The API can also be used to evaluate the quality of
the documents or to carry out further annotations.


## Contributing

All contributors are required to "sign-off" their commits (using `git
commit -s`) to indicate that they have agreed to the [Developer
Certificate of Origin](https://developercertificate.org/).


## License

This project is licensed under the [European Union Public License 1.2].
The [test data] were taken from the Wikipedia articles [DNB], [TIB] and
[ZBW] and are licensed under [CC BY-SA].



[Annif]: https://annif.org
[German National Library]: https://www.dnb.de
[European Union Public License 1.2]: ./LICENSE
[test data]: ./crates/datashed/tests/data
[DNB]: https://de.wikipedia.org/wiki/Deutsche_Nationalbibliothek
[TIB]: https://de.wikipedia.org/wiki/TIB_%E2%80%93_Leibniz-Informationszentrum_Technik_und_Naturwissenschaften
[ZBW]: https://en.wikipedia.org/wiki/German_National_Library_of_Economics
[CC BY-SA]: https://creativecommons.org/licenses/by-sa/4.0/
[datashed]: https://deutsche-nationalbibliothek.github.io/dataset-tools/getting-started/datashed.html
