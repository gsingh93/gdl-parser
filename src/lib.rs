//! This crate provides a function for parsing a GDL description to an AST. To get an AST, call the
//! `parse` function with the string representing the description. The AST is based off the
//! [AST](http://www.ggp.org/developers/gdl.html) used in
//! [GGP Base](https://github.com/ggp-org/ggp-base).

#![feature(plugin, collections, str_char)]
#![plugin(peg_syntax_ext)]

extern crate rustc_serialize;

use gdl::description;
use self::Clause::{RuleClause, SentenceClause};
use self::Sentence::{PropSentence, RelSentence};
use self::Literal::{NotLit, DistinctLit, OrLit, PropLit, RelLit};
use self::Term::{VarTerm, FuncTerm, ConstTerm};

pub mod visitor;

peg_file! gdl("grammar.rustpeg");

/// Parse a GDL string to a `Description`. Panics if the description is invalid.
pub fn parse(gdl: &str) -> Description {
    match description(gdl) {
        Ok(d) => d,
        Err(e) => panic!("{}", e)
    }
}

/// A GDL description. Contains a vector of `Clause`s, which are the top-level statements in
/// a GDL description.
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Description {
    pub clauses: Vec<Clause>
}

impl Description {
    /// Constructs a new description from a vector of `Clause`s
    pub fn new(clauses: Vec<Clause>) -> Description {
        Description { clauses: clauses }
    }
}

impl ToString for Description {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for clause in self.clauses.iter() {
            s.push_str(&clause.to_string());
            s.push(' ');
        }
        let len = s.len();
        s.remove(len - 1);
        s
    }
}

/// A top level statement in GDL. The only types of top level statements are `Rule`s and
/// `Sentence`s
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Clause {
    // A rule clause
    RuleClause(Rule),

    // A sentence clause
    SentenceClause(Sentence)
}

impl ToString for Clause {
    fn to_string(&self) -> String {
        match self {
            &RuleClause(ref r) => r.to_string(),
            &SentenceClause(ref s) => s.to_string(),
        }
    }
}

/// A GDL rule contains a head `Sentence` that is implied by all the `Literal`s in the body
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

impl ToString for Rule {
    fn to_string(&self) -> String {
        let mut s = String::from_str("(<= ");
        s.push_str(&self.head.to_string());
        for arg in self.body.iter() {
            s.push(' ');
            s.push_str(&arg.to_string());
        }
        s.push(')');
        s
    }
}

/// A GDL sentence is like a `Rule` without a body. The two types of `Sentence`s are `Relation`s
/// and `Proposition`s
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Sentence {
    /// A proposition sentence
    PropSentence(Proposition),

    /// A relation sentence
    RelSentence(Relation)
}

impl Sentence {
    /// Returns the name of the sentence
    pub fn name(&self) -> &Constant {
        match self {
            &PropSentence(ref p) => &p.name,
            &RelSentence(ref r) => &r.name
        }
    }
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

/// A GDL literal
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Literal {
    NotLit(Not),
    OrLit(Or),
    DistinctLit(Distinct),
    PropLit(Proposition),
    RelLit(Relation)
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        match self {
            &NotLit(ref n) => n.to_string(),
            &OrLit(ref o) => o.to_string(),
            &DistinctLit(ref d) => d.to_string(),
            &PropLit(ref p) => p.to_string(),
            &RelLit(ref r) => r.to_string(),
        }
    }
}

/// A GDL term is either a variable, a function, or a constant
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Term {
    VarTerm(Variable),
    FuncTerm(Function),
    ConstTerm(Constant)
}

impl Term {
    /// Returns the name of the term
    pub fn name(&self) -> &Constant {
        match self {
            &VarTerm(ref v) => &v.name,
            &FuncTerm(ref f) => &f.name,
            &ConstTerm(ref c) => c
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

/// A proposition is a `Relation` with no body; it only has a name.
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Proposition {
    pub name: Constant
}

impl Proposition {
    pub fn new<T: Into<Constant>>(name: T) -> Proposition {
        Proposition { name: name.into() }
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

/// A GDL relation
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Relation {
    pub name: Constant,
    pub args: Vec<Term>
}

impl Relation {
    /// Consruct a new `Relation` given a `name` and a list of `args`
    pub fn new<T: Into<Constant>>(name: T, args: Vec<Term>) -> Relation {
        Relation { name: name.into(), args: args }
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
            s.push_str(&arg.to_string());
        }
        s.push(')');
        s
    }
}

/// A not literal
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

impl ToString for Not {
    fn to_string(&self) -> String {
        let mut s = String::from_str("(not ");
        s.push_str(&self.lit.to_string());
        s.push(')');
        s
    }
}

/// An or literal
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

impl ToString for Or {
    fn to_string(&self) -> String {
        let mut s = String::from_str("(or");
        for l in self.lits.iter() {
            s.push(' ');
            s.push_str(&l.to_string());
        }
        s.push(')');
        s
    }
}

/// A distinct literal
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

impl ToString for Distinct {
    fn to_string(&self) -> String {
        let mut s = String::from_str("(distinct ");
        s.push_str(&self.term1.to_string());
        s.push(' ');
        s.push_str(&self.term2.to_string());
        s.push(')');
        s
    }
}

/// A variable term
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Variable {
    pub name: Constant
}

impl Variable {
    pub fn new<T: Into<Constant>>(name: T) -> Variable {
        Variable { name: name.into() }
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

/// A function term
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Function {
    pub name: Constant,
    pub args: Vec<Term>
}

impl Function {
    pub fn new<T: Into<Constant>>(name: T, args: Vec<Term>) -> Function {
        Function { name: name.into(), args: args }
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
            s.push_str(&arg.to_string());
        }
        s.push(')');
        s
    }
}

/// A GDL constant
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

impl<'a> Into<Constant> for &'a str {
    fn into(self) -> Constant {
        Constant::new(self.to_string())
    }
}
