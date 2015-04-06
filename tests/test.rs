extern crate gdl_parser;
extern crate rustc_serialize;

use gdl_parser::parse;
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
