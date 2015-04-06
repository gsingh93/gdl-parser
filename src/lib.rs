#![feature(plugin, collections, str_char)]
#![plugin(peg_syntax_ext)]

extern crate rustc_serialize;

use gdl::program;

peg_file! gdl("grammar.rustpeg");

#[derive(Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Program {
    pub clauses: Vec<Clause>
}

#[derive(Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Clause {
    RuleClause(Rule),
    SentenceClause(Sentence)
}

#[derive(Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Rule {
    head: Sentence,
    body: Vec<Literal>
}

#[derive(Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Sentence {
    PropSentence(Proposition),
    RelSentence(Relation)
}

#[derive(Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Literal {
    NotLit(Not),
    OrLit(Or),
    DistinctLit(Distinct),
    PropLit(Proposition),
    RelLit(Relation)
}

#[derive(Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Term {
    VarTerm(Variable),
    FuncTerm(Function),
    ConstTerm(Constant)
}

#[derive(Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Proposition {
    name: Constant
}

#[derive(Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Relation {
    name: Constant,
    args: Vec<Term>
}

#[derive(Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Not {
    lit: Box<Literal>
}

#[derive(Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Or {
    lits: Vec<Literal>
}

#[derive(Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Distinct {
    term1: Term,
    term2: Term
}

#[derive(Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Variable {
    name: Constant
}

#[derive(Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Function {
    name: Constant,
    args: Vec<Term>
}

#[derive(Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Constant {
    name: String
}

pub fn parse(s: &str) -> Program {
    match program(s) {
        Ok(p) => p,
        Err(e) => panic!("{}", e)
    }
}
