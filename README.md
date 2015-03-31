GDL Parser
==========

This is a parser for GDL (game description language). GDL is a subset of KIF (knowledge interchange format). This parser focuses on GDL and not KIF for the purpose of GGP (general game playing). It converts a GDL string to an AST but does not do any semantic analysis on this AST. It makes use of the [rust-peg](https://github.com/kevinmehall/rust-peg) parser generator.

You can find the specification for GDL [here](http://logic.stanford.edu/classes/cs227/2013/readings/gdl_spec.pdf) and the specification for KIF [here](http://logic.stanford.edu/kif/Hypertext/kif-manual.html).

Installation
------------

You can install the package from [crates.io](https://crates.io/) by adding the following to the dependencies section of your `Cargo.toml`:

```
gdl-parser = "*"
```

Usage
-----

```
extern crate gdl_parser;
use gdl_parser::parse;

println!("{:?}", parse("(role red) (role black)"));
```
