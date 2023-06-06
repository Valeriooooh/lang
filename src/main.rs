extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;
#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;
use std::fs;

fn main() {
    let unparsed_file = fs::read_to_string("test.lang").expect("cannot read file");
    let file = CSVParser::parse(Rule::file, &unparsed_file)
        .unwrap()
        .next()
        .unwrap();
    // .expect("unsuccessful parse") // unwrap the parse result
}
