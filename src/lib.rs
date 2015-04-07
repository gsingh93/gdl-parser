#![feature(plugin, collections, str_char)]
#![plugin(peg_syntax_ext)]

extern crate rustc_serialize;

use gdl::description;

peg_file! gdl("grammar.rustpeg");

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Description {
    pub clauses: Vec<Clause>
}

impl Description {
    pub fn new(clauses: Vec<Clause>) -> Description {
        Description { clauses: clauses }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Clause {
    RuleClause(Rule),
    SentenceClause(Sentence)
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Rule {
    pub head: Sentence,
    pub body: Vec<Literal>
}

impl Rule {
    pub fn new(head: Sentence, body: Vec<Literal>) -> Rule {
        Rule { head: head, body: body }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Sentence {
    PropSentence(Proposition),
    RelSentence(Relation)
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Literal {
    NotLit(Not),
    OrLit(Or),
    DistinctLit(Distinct),
    PropLit(Proposition),
    RelLit(Relation)
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Term {
    VarTerm(Variable),
    FuncTerm(Function),
    ConstTerm(Constant)
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Proposition {
    pub name: Constant
}

impl Proposition {
    pub fn new(name: Constant) -> Proposition {
        Proposition { name: name}
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Relation {
    pub name: Constant,
    pub args: Vec<Term>
}

impl Relation {
    pub fn new(name: Constant, args: Vec<Term>) -> Relation {
        Relation { name: name, args: args }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Not {
    pub lit: Box<Literal>
}

impl Not {
    pub fn new(lit: Box<Literal>) -> Not {
        Not { lit: lit }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Or {
    pub lits: Vec<Literal>
}

impl Or {
    pub fn new(lits: Vec<Literal>) -> Or {
        Or { lits: lits }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Distinct {
    pub term1: Term,
    pub term2: Term
}

impl Distinct {
    pub fn new(term1: Term, term2: Term) -> Distinct {
        Distinct { term1: term1, term2: term2 }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Variable {
    pub name: Constant
}

impl Variable {
    pub fn new(name: Constant) -> Variable {
        Variable { name: name }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Function {
    pub name: Constant,
    pub args: Vec<Term>
}

impl Function {
    pub fn new(name: Constant, args: Vec<Term>) -> Function {
        Function { name: name, args: args }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Constant {
    pub name: String
}

impl Constant {
    pub fn new(name: String) -> Constant {
        Constant { name: name }
    }
}

pub fn parse(s: &str) -> Description {
    match description(s) {
        Ok(d) => d,
        Err(e) => panic!("{}", e)
    }
}
