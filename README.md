GDL Parser
==========

This is a parser for GDL (game description language). GDL is a subset [Datalog](https://en.wikipedia.org/wiki/Datalog), but when used for GGP (general game playing) it is sent in KIF (knowledge interchange format). This parser focuses on GDL and not KIF for the purpose of GGP and is currently being used in [ggp-rs](https://github.com/gsingh93/ggp-rs).

The parser converts a GDL string to an AST but does not do any semantic analysis on this AST. It makes use of the [rust-peg](https://github.com/kevinmehall/rust-peg) parser generator. The AST is based off of the AST used in [GGP Base](https://github.com/ggp-org/ggp-base) which can be seen [here](http://www.ggp.org/developers/gdl.html).

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

Documentation
-------------

You can find the API docs [here](https://gsingh93.github.io/gdl-parser/doc/gdl_parser/).

Grammar
-------

Here is the EBNF of the grammar. I came up with this EBNF myself by examining the parsing code in GGP Base, so if there are any bugs please report them.

```
description := { rule | sentence }

rule := '(' '<=' sentence { literal } ')'

sentence := prop_lit | ( '(' rel_lit ')' )

literal := ( '(' (or_lit | not_lit | distinct_lit | rel_lit) ')' ) | prop_lit
not_lit := 'not' literal
or_lit := 'or' { literal }
distinct_lit := 'distinct' term term
prop_lit := constant
rel_lit := constant { term }

term := ( '(' func_term ')' ) | var_term | const_term
func_term := constant { term }
var_term := '?' constant
const_term := constant

(* ident is any string of letters, digits, and underscores *)
constant := ident
```

License
-------

[MIT](https://github.com/gsingh93/gdl-parser/blob/master/LICENSE.txt)
