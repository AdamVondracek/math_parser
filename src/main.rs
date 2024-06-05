#![allow(dead_code)]

include!("lexer.rs");
include!("parsing.rs");

fn main() 
{
  let symbols = ['+','-','*','/','=','(',')', '.'];
  let symbol_set: HashSet<char> = HashSet::from(symbols);
  let input = String::from("69+(420-9*13)=420");
  let split_input = lexer(&input, &symbol_set);
  for token in split_input
  {
    math_parser(&token);
  }
}