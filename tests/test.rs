extern crate gdl_parser;
extern crate rustc_serialize;

use gdl_parser::{parse, Constant, Proposition, Relation};
use gdl_parser::Sentence::{PropSentence, RelSentence};

use rustc_serialize::json;

use std::fs::File;
use std::io::Read;

#[test]
fn test() {
    let mut gdl = String::new();
    let f = File::open("tests/test-alquerque.gdl");
    f.unwrap().read_to_string(&mut gdl).ok().expect("Unable to read GDL file");

    let mut json = String::new();
    let f = File::open("tests/test-alquerque.json");
    f.unwrap().read_to_string(&mut json).ok().expect("Unable to read JSON file");

    assert_eq!(parse(&gdl), json::decode(&json).unwrap());
}

#[test]
fn test_to_string() {
    let sentence = PropSentence(Proposition::new(Constant::new("p".to_string())));
    assert_eq!(sentence.to_string(), "p".to_string());

    let sentence = RelSentence(
        Relation::new(Constant::new("p".to_string()),
                      vec![Constant::new("a".to_string()).into(),
                           Constant::new("b".to_string()).into()]));
    assert_eq!(sentence.to_string(), "(p a b)".to_string());
}
