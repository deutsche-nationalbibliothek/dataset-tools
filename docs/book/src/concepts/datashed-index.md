# Datashed Index

## Usage

By default, the index is in [Apache Arrow] format (with [zstd]
compression). Since the index can sometimes contain a very large
number of rows, an efficient DataFrame library such as [polars],
[r-polars], or [tidypolars] is recommended for use in data analysis.

With [polars] (Python), the index can be read and processed as follows:

```python
>>> import polars as pl
>>> 
>>> index = pl.read_ipc("index.ipc", memory_map=False)
>>> index.glimpse(max_items_per_column=2)
Rows: 14986358
Columns: 12
$ path          <str> '000/1000217000.txt', '000/1000246000.txt'
$ hash          <str> 'd8d939da92bb', '400097e501fa'
$ ppn           <str> '1000217000', '1000246000'
$ genre        <enum> nonfiction, nonfiction
$ group        <enum> monograph, monograph
$ doctype      <enum> doctoral-thesis, doctoral-thesis
$ chars         <u64> 306107, 285848
$ size          <u64> 306581, 289161
$ lang    <struct[2]> {'code': 'eng', 'score': 1.0}, {'code': 'ger', 'score': 1.0}
$ lfreq         <f64> 0.05100938606121107, 0.032885320167713325
$ alpha         <f64> 0.7319172707582643, 0.7772277574095323
$ mtime         <u64> 1744879787, 1744879702
```

> [!NOTE]
> The optional `ppn` column contains the document's filename (without
the file extension) and was added using the [`--filename-column`] option.

If the Datashed's HTTP endpoint is active (see the [serve] command), the
current index can be accessed via the route `/index.ipc`.

## Columns

### Path

The `path` column contains the path to the document, relative to the
datashed's `data/` directory.

### Hash

The `hash` column contains a six-byte hexadecimal hash of the document.
The hash value is derived from the document's [SHA256] checksum, by taking
the first six bytes and format them as a hexadecimal string. Documents
that have the same `hash` value are very likely to have the same content
and might be treated as duplicates.

### Chars

The total number of [Unicode scalar values] in the document is contained
in the `chars` column. Note that if invalid UTF-8 is encountered, then
the Unicode replacement codepoint is yielded instead.

### Document Size

The `size` column contains the size of the document in bytes.

### Language

The language of the document is given as an [ISO 639-2 (B)] language
code in the `lang_code` column. The `lang_score` column contains the
confidence value. If the index is in [Apache Arrow] format, which is the
default, the two columns are combined into one column `lang` with fields
`code` and `score`. Language detection is performed with [lingua] in
high-accuracy mode.


### Letter Frequency

The `lfreq` score contains a measure of how far the document deviates
from the letter distribution of the respective language. To get the
value, a vector \\(x\\) of relative letter frequencies is first calculated
over the characters of a fixed alphabet. Then the [euclidian distance]
to the reference vector \\(y\\) of the respective language is calculated
(\\(\ell^2\\)-norm).

$$
\text{lfreq} \triangleq \left \lVert x - y \right \rVert_2
$$

Note that the range of the score is from \\(0\\) to \\(\sqrt 2\\).
Letters that are not part of the language's alphabet are ignored. So far
only English and German are supported. If the language of the document
is not supported, the `lfreq` value is set to `null`.

> [!NOTE]
> If you need the support of another language, please create a [GitHub
> issue] with a reference to the alphabet of the language and a reference
> vector containing the relative frequencies.

### Alpha

The `alpha` score of a document is the ratio of alphabetic characters to
the total number of characters. An alphabetic character is a character
which satisfy the _Alphabetic_ property of the [Unicode Standard]
described in Chapter 4 (Character Properties). The score is defined as

$$
\text{alpha} \triangleq \frac{1}{N}\sum_{i = 1}^{N} \mathbf{1}_A(c_i)
$$

where \\(N\\) is total number of characters of the document, \\(c_i\\) is the
i-th character of the document, \\(A\\) is the subset of all characters,
which satisfy the _Alphabetic_ property and \\(\mathbf{1}_A\\) is the
indicator function, which returns 1 if the i-th character is alphabetic
and otherwise 0. The range of the function is \\([0, 1]\\) and the score of
an empty  document is defined to \\(0.0\\).

A very low `alpha` value may indicate a corrupt document.

### Last Modification

Finally, the `mtime` column contains the Unix timestamp of the
document's last modification.



[Apache Arrow]: https://arrow.apache.org
[euclidian distance]: https://en.wikipedia.org/wiki/Euclidean_distance
[`--filename-column`]: ../reference/datashed/commands/datashed-index.md#--filename-column-column
[GitHub Issue]: https://github.com/deutsche-nationalbibliothek/dataset-tools/issues/new
[ISO 639-2 (B)]: https://en.wikipedia.org/wiki/List_of_ISO_639-2_codes
[lingua]: https://github.com/pemistahl/lingua-rs
[polars]: https://pola.rs
[r-polars]: https://pola-rs.github.io/r-polars
[serve]: ../reference/datashed/commands/datashed-serve.md
[SHA256]: https://en.wikipedia.org/wiki/SHA-2
[tidypolars]: https://tidypolars.etiennebacher.com
[Unicode scalar values]: https://www.unicode.org/glossary/#unicode_scalar_value
[Unicode Standard]: https://www.unicode.org/versions/latest
[zstd]: https://en.wikipedia.org/wiki/Zstd
