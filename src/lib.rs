//! This crate provides a function for parsing a GDL description to an AST. To get an AST, call the
//! `parse` function with the string representing the description. The AST is based off the
//! [AST](http://www.ggp.org/developers/gdl.html) used in
//! [GGP Base](https://github.com/ggp-org/ggp-base).

extern crate rustc_serialize;

mod gdl;

use std::fmt::{Display, Formatter, Error};

use gdl::description;
use self::Clause::{RuleClause, SentenceClause};
use self::Sentence::{PropSentence, RelSentence};
use self::Literal::{NotLit, DistinctLit, OrLit, PropLit, RelLit};
use self::Term::{VarTerm, FuncTerm, ConstTerm};

pub mod visitor;

/// Parse a GDL string to a `Description`. Panics if the description is invalid.
pub fn parse(gdl: &str) -> Description {
    match description(gdl) {
        Ok(d) => d,
        Err(e) => panic!("{}", e)
    }
}

/// A GDL description. Contains a vector of `Clause`s, which are the top-level statements in
/// a GDL description.
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable, Ord, PartialOrd)]
pub struct Description {
    pub clauses: Vec<Clause>
}

impl Description {
    /// Constructs a new description from a vector of `Clause`s
    pub fn new(clauses: Vec<Clause>) -> Description {
        Description { clauses: clauses }
    }
}

impl Display for Description {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut s = String::new();
        for clause in self.clauses.iter() {
            s.push_str(&clause.to_string());
            s.push(' ');
        }
        let len = s.len();
        s.remove(len - 1);
        write!(f, "{}", s)
    }
}

/// A top level statement in GDL. The only types of top level statements are `Rule`s and
/// `Sentence`s
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable, Ord, PartialOrd)]
pub enum Clause {
    // A rule clause
    RuleClause(Rule),

    // A sentence clause
    SentenceClause(Sentence)
}

impl Display for Clause {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &RuleClause(ref r) => Display::fmt(r, f),
            &SentenceClause(ref s) => Display::fmt(s, f)
        }
    }
}

/// A GDL rule contains a head `Sentence` that is implied by all the `Literal`s in the body
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable, Ord, PartialOrd)]
pub struct Rule {
    pub head: Sentence,
    pub body: Vec<Literal>
}

impl Rule {
    pub fn new(head: Sentence, body: Vec<Literal>) -> Rule {
        Rule { head: head, body: body }
    }
}

impl Into<Clause> for Rule {
    fn into(self) -> Clause {
        RuleClause(self)
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut s = String::from(&*self.head.to_string());
        for arg in self.body.iter() {
            s.push(' ');
            s.push_str(&arg.to_string());
        }
        write!(f, "(<= {})", s)
    }
}

/// A GDL sentence is like a `Rule` without a body. The two types of `Sentence`s are `Relation`s
/// and `Proposition`s
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable, Ord, PartialOrd)]
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

impl Into<Clause> for Sentence {
    fn into(self) -> Clause {
        SentenceClause(self)
    }
}

impl Display for Sentence {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &PropSentence(ref p) => Display::fmt(p, f),
            &RelSentence(ref r) => Display::fmt(r, f)
        }
    }
}

/// A GDL literal
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable, Ord, PartialOrd)]
pub enum Literal {
    NotLit(Not),
    OrLit(Or),
    DistinctLit(Distinct),
    PropLit(Proposition),
    RelLit(Relation)
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &NotLit(ref n) => Display::fmt(n, f),
            &OrLit(ref o) => Display::fmt(o, f),
            &DistinctLit(ref d) => Display::fmt(d, f),
            &PropLit(ref p) => Display::fmt(p, f),
            &RelLit(ref r) => Display::fmt(r, f),
        }
    }
}

/// A GDL term is either a variable, a function, or a constant
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable, Ord, PartialOrd)]
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

impl Display for Term {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &VarTerm(ref v) => Display::fmt(v, f),
            &FuncTerm(ref func) => Display::fmt(func, f),
            &ConstTerm(ref c) => Display::fmt(c, f)
        }
    }
}

/// A proposition is a `Relation` with no body; it only has a name.
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable, Ord, PartialOrd)]
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

impl Display for Proposition {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Display::fmt(&self.name, f)
    }
}

/// A GDL relation
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable, Ord, PartialOrd)]
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

impl Display for Relation {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut s = String::from(&*self.name.name);
        for arg in self.args.iter() {
            s.push(' ');
            s.push_str(&arg.to_string());
        }
        write!(f, "({})", s)
    }
}

/// A not literal
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable, Ord, PartialOrd)]
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

impl Display for Not {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "(not {})", self.lit)
    }
}

/// An or literal
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable, Ord, PartialOrd)]
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

impl Display for Or {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut s = String::new();
        for l in self.lits.iter() {
            s.push(' ');
            s.push_str(&l.to_string());
        }
        write!(f, "(or {})", s)
    }
}

/// A distinct literal
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable, Ord, PartialOrd)]
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

impl Display for Distinct {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "(distinct {} {})", self.term1, self.term2)
    }
}

/// A variable term
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable, Ord, PartialOrd)]
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

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut s = self.name.to_string();
        s.insert(0, '?');
        write!(f, "?{}", self.name)
    }
}

/// A function term
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable, Ord, PartialOrd)]
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

impl Display for Function {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut s = String::from(&*self.name.name);
        for arg in self.args.iter() {
            s.push(' ');
            s.push_str(&arg.to_string());
        }
        write!(f, "({})", s)
    }
}

/// A GDL constant
#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable, RustcEncodable, Ord, PartialOrd)]
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

impl Display for Constant {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.name)
    }
}

impl<'a> Into<Constant> for &'a str {
    fn into(self) -> Constant {
        Constant::new(self.to_string())
    }
}
