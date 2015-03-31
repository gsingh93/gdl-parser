GDL Parser
==========

This is a parser for GDL (game description language). GDL is a subset of KIF (knowledge interchange format). This parser focuses on GDL and not KIF for the purpose of GGP (general game playing). It converts a GDL string to an AST but does not do any semantic analysis on this AST. It makes use of the [rust-peg](https://github.com/kevinmehall/rust-peg) parser generator.

You can find the specification for GDL [here](http://logic.stanford.edu/classes/cs227/2013/readings/gdl_spec.pdf) and the specification for KIF [here](http://logic.stanford.edu/kif/Hypertext/kif-manual.html).

Usage
-----

```
let parser = GDLParser::new();
println!("{:?}", parser.parse("(role red) (role black)"));
```
