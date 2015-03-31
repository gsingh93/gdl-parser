#![feature(plugin, collections, str_char)]
#![plugin(peg_syntax_ext)]

extern crate rustc_serialize;

use gdl::program;

peg_file! gdl("grammar.rustpeg");

#[derive(Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Program {
    sexprs: Vec<SExpr>
}

#[derive(Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct SExpr {
    terms: Vec<Term>
}

#[derive(Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Term {
    RuleOp,
    Constant(String),
    Variable(String),
    ExprTerm(SExpr),
    Num(u8)
}

pub fn parse(s: &str) -> Program {
    match program(s) {
        Ok(p) => p,
        Err(e) => panic!("{}", e)
    }
}
