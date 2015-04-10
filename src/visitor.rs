use {Description, Sentence, Proposition, Relation, Literal, Or, Not, Distinct, Function, Rule,
          Variable, Constant, Clause, Term};
use Clause::{RuleClause, SentenceClause};
use Sentence::{PropSentence, RelSentence};
use Term::{ConstTerm, FuncTerm, VarTerm};
use Literal::{OrLit, NotLit, DistinctLit, PropLit, RelLit};

/// A visitor for the AST. Each function will get called when that respective AST node is visited
pub trait Visitor {
    fn visit_clause(&mut self, _: &mut Clause) {}

    fn visit_rule(&mut self, _: &mut Rule) {}

    fn visit_sentence(&mut self, _: &mut Sentence) {}

    fn visit_proposition(&mut self, _: &mut Proposition) {}

    fn visit_relation(&mut self, _: &mut Relation) {}

    fn visit_literal(&mut self, _: &mut Literal) {}

    fn visit_term(&mut self, _: &mut Term) {}

    fn visit_constant(&mut self, _: &mut Constant) {}

    fn visit_or(&mut self, _: &mut Or) {}

    fn visit_not(&mut self, _: &mut Not) {}

    fn visit_distinct(&mut self, _: &mut Distinct) {}

    fn visit_variable(&mut self, _: &mut Variable) {}

    fn visit_function(&mut self, _: &mut Function) {}
}

/// Performs a post-order traversal of a GDL description
pub fn visit<V: Visitor>(desc: &mut Description, visitor: &mut V) {
    for clause in desc.clauses.iter_mut() {
        visit_clause(clause, visitor);
    }
}

/// Performs a post-order traversal of a GDL clause
pub fn visit_clause<V: Visitor>(clause: &mut Clause, visitor: &mut V) {
    match clause {
        &mut RuleClause(ref mut r) => visit_rule(r, visitor),
        &mut SentenceClause(ref mut s) => visit_sentence(s, visitor)
    }
    visitor.visit_clause(clause);
}

/// Performs a post-order traversal of a GDL rule
pub fn visit_rule<V: Visitor>(rule: &mut Rule, visitor: &mut V) {
    visit_sentence(&mut rule.head, visitor);
    for l in rule.body.iter_mut() {
        visit_literal(l, visitor);
    }
    visitor.visit_rule(rule);
}

/// Performs a post-order traversal of a GDL sentence
pub fn visit_sentence<V: Visitor>(sentence: &mut Sentence, visitor: &mut V) {
    match sentence {
        &mut PropSentence(ref mut p) => visit_proposition(p, visitor),
        &mut RelSentence(ref mut r) => visit_relation(r, visitor)
    }
    visitor.visit_sentence(sentence);
}

/// Performs a post-order traversal of a GDL proposition
pub fn visit_proposition<V: Visitor>(proposition: &mut Proposition, visitor: &mut V) {
    visit_constant(&mut proposition.name, visitor);
    visitor.visit_proposition(proposition)
}

/// Performs a post-order traversal of a GDL relation
pub fn visit_relation<V: Visitor>(relation: &mut Relation, visitor: &mut V) {
    visit_constant(&mut relation.name, visitor);
    for t in relation.args.iter_mut() {
        visit_term(t, visitor)
    }
    visitor.visit_relation(relation)
}

/// Performs a post-order traversal of a GDL literal
pub fn visit_literal<V: Visitor>(literal: &mut Literal, visitor: &mut V) {
    match literal {
        &mut OrLit(ref mut or) => visit_or(or, visitor),
        &mut NotLit(ref mut not) => visit_not(not, visitor),
        &mut DistinctLit(ref mut distinct) => visit_distinct(distinct, visitor),
        &mut RelLit(ref mut rel) => visit_relation(rel, visitor),
        &mut PropLit(ref mut prop) => visit_proposition(prop, visitor)
    }
    visitor.visit_literal(literal);
}

/// Performs a post-order traversal of a GDL term
pub fn visit_term<V: Visitor>(term: &mut Term, visitor: &mut V) {
    match term {
        &mut ConstTerm(ref mut c) => visit_constant(c, visitor),
        &mut FuncTerm(ref mut f) => visit_function(f, visitor),
        &mut VarTerm(ref mut v) => visit_variable(v, visitor)
    }
    visitor.visit_term(term);
}

/// Performs a post-order traversal of a GDL constant
pub fn visit_constant<V: Visitor>(constant: &mut Constant, visitor: &mut V) {
    visitor.visit_constant(constant);
}

/// Performs a post-order traversal of a GDL or literal
pub fn visit_or<V: Visitor>(or: &mut Or, visitor: &mut V) {
    for l in or.lits.iter_mut() {
        visit_literal(l, visitor);
    }
    visitor.visit_or(or);
}

/// Performs a post-order traversal of a GDL not literal
pub fn visit_not<V: Visitor>(not: &mut Not, visitor: &mut V) {
    visit_literal(&mut not.lit, visitor);
    visitor.visit_not(not);
}

/// Performs a post-order traversal of a GDL distinct literal
pub fn visit_distinct<V: Visitor>(distinct: &mut Distinct, visitor: &mut V) {
    visit_term(&mut distinct.term1, visitor);
    visit_term(&mut distinct.term2, visitor);
    visitor.visit_distinct(distinct);
}

/// Performs a post-order traversal of a GDL variable
pub fn visit_variable<V: Visitor>(variable: &mut Variable, visitor: &mut V) {
    visit_constant(&mut variable.name, visitor);
    visitor.visit_variable(variable);
}

/// Performs a post-order traversal of a GDL function
pub fn visit_function<V: Visitor>(function: &mut Function, visitor: &mut V) {
    visit_constant(&mut function.name, visitor);
    for t in function.args.iter_mut() {
        visit_term(t, visitor)
    }
    visitor.visit_function(function)
}
