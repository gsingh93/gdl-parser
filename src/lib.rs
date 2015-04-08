#![feature(plugin, collections, str_char)]
#![plugin(peg_syntax_ext)]

extern crate rustc_serialize;

use gdl::description;
use self::Sentence::{PropSentence, RelSentence};
use self::Literal::{NotLit, DistinctLit, OrLit, PropLit, RelLit};
use self::Term::{VarTerm, FuncTerm, ConstTerm};

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

impl Into<Literal> for Sentence {
    fn into(self) -> Literal {
        match self {
            PropSentence(p) => PropLit(p),
            RelSentence(r) => RelLit(r)
        }
    }
}

impl Into<Rule> for Sentence {
    fn into(self) -> Rule {
        Rule::new(self, Vec::new())
    }
}

impl ToString for Sentence {
    fn to_string(&self) -> String {
        match self {
            &PropSentence(ref p) => p.to_string(),
            &RelSentence(ref r) => r.to_string()
        }
    }
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

impl Term {
    pub fn name(&self) -> &str {
        match self {
            &VarTerm(ref v) => &v.name.name,
            &FuncTerm(ref f) => &f.name.name,
            &ConstTerm(ref c) => &c.name
        }
    }
}

impl ToString for Term {
    fn to_string(&self) -> String {
        match self {
            &VarTerm(ref v) => v.to_string(),
            &FuncTerm(ref f) => f.to_string(),
            &ConstTerm(ref c) => c.to_string()
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Proposition {
    pub name: Constant
}

impl Proposition {
    pub fn new(name: Constant) -> Proposition {
        Proposition { name: name }
    }
}

impl Into<Literal> for Proposition {
    fn into(self) -> Literal {
        PropLit(self)
    }
}

impl Into<Sentence> for Proposition {
    fn into(self) -> Sentence {
        PropSentence(self)
    }
}

impl Into<Relation> for Proposition {
    fn into(self) -> Relation {
        Relation::new(self.name, Vec::new())
    }
}

impl ToString for Proposition {
    fn to_string(&self) -> String {
        self.name.to_string()
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

impl Into<Literal> for Relation {
    fn into(self) -> Literal {
        RelLit(self)
    }
}

impl Into<Sentence> for Relation {
    fn into(self) -> Sentence {
        RelSentence(self)
    }
}

impl ToString for Relation {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push('(');
        s.push_str(&self.name.name);
        for arg in self.args.iter() {
            s.push(' ');
            match arg {
                &FuncTerm(ref f) => s.push_str(&f.to_string()),
                t => s.push_str(t.name())
            }
        }
        s.push(')');
        s
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

impl Into<Literal> for Not {
    fn into(self) -> Literal {
        NotLit(self)
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

impl Into<Literal> for Or {
    fn into(self) -> Literal {
        OrLit(self)
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

impl Into<Literal> for Distinct {
    fn into(self) -> Literal {
        DistinctLit(self)
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

impl Into<Term> for Variable {
    fn into(self) -> Term {
        VarTerm(self)
    }
}

impl ToString for Variable {
    fn to_string(&self) -> String {
        let mut s = self.name.to_string();
        s.insert(0, '?');
        s
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

impl Into<Term> for Function {
    fn into(self) -> Term {
        FuncTerm(self)
    }
}

impl ToString for Function {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push('(');
        s.push_str(&self.name.name);
        for arg in self.args.iter() {
            s.push(' ');
            match arg {
                &FuncTerm(ref f) => s.push_str(&f.to_string()),
                t => s.push_str(t.name())
            }
        }
        s.push(')');
        s
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Constant {
    pub name: String
}

impl Constant {
    pub fn new<T: Into<String>>(name: T) -> Constant {
        Constant { name: name.into() }
    }
}

impl Into<Term> for Constant {
    fn into(self) -> Term {
        ConstTerm(self)
    }
}

impl ToString for Constant {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

pub fn parse(s: &str) -> Description {
    match description(s) {
        Ok(d) => d,
        Err(e) => panic!("{}", e)
    }
}
