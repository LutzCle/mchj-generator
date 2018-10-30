mchj-generator
========

mchj-generator is a Rust wrapper for Cagri Balkesen's join data generator.
The generator features in Balkesen et al. "Multi-Core, Main-Memory Joins: Sort vs. Hash Revisited", PVLDB '13.
The original code can be found [here](https://www.systems.ethz.ch/sites/default/files/file/PublishedCode/multicore-hashjoins-0_2_tar.gz)
on the [ETH Zurich System's Group website](https://www.systems.ethz.ch/projects/paralleljoins).

This crate's purpose is to make the data generator easy to use for scientific reproducability of the paper's results.

## Copyright notice

Note that although the provided Rust wrapper is licensed under Apache 2.0,
the original code is copyright of Cagri Balkesen and NOT under Apache 2.0.
This applies to all C header and source files in the libmchj-generator directory.

## Citation

If you use this crate for scientific purposes, kindly cite Balkesen et al.:

```tex
@article{DBLP:journals/pvldb/BalkesenATO13,
  author    = {Cagri Balkesen and
               Gustavo Alonso and
               Jens Teubner and
               M. Tamer {\"{O}}zsu},
  title     = {Multi-Core, Main-Memory Joins: Sort vs. Hash Revisited},
  journal   = {{PVLDB}},
  volume    = {7},
  number    = {1},
  pages     = {85--96},
  year      = {2013},
  url       = {http://www.vldb.org/pvldb/vol7/p85-balkesen.pdf},
  doi       = {10.14778/2732219.2732227},
  timestamp = {Thu, 16 Aug 2018 11:33:44 +0200},
  biburl    = {https://dblp.org/rec/bib/journals/pvldb/BalkesenATO13},
  bibsource = {dblp computer science bibliography, https://dblp.org}
}
```

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
mchj-generator = { git = "https://github.com/LutzCle/mchj-generator.git" }
```

Next, add this to your crate root:

```rust
extern crate mchj_generator;
```

The documentation can be generated and opened in your web browser with:

```bash
cargo doc --open
```
