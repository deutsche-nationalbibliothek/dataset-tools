# Installation

## From Source

If a Rust toolchain is available, the dataset tools can be installed
using the Rust package manager [cargo]. The project requires a Rust
compiler with a minimum version of 1.89. Use the following command to
install the program with the default features:

```console
$ cargo install --path crates/datashed-cli
$ cargo install --path crates/dataset-cli
```

The binary can be built with the following features as needed:

`performant`
: This feature activates optimizations aimed at improving performance.
This includes, for example, the activation of SIMD or a more aggressive
inline strategy.

