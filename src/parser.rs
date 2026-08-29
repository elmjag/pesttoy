use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "toy.pest"]
struct ToyParser;

pub fn parse(input: &str) -> Pairs<'_, Rule> {
    ToyParser::parse(Rule::file, input).unwrap()
}
