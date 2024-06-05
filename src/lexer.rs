use std::collections::HashSet;
fn lexer(input: &str, symbols: &HashSet<char>) -> Vec<String>
{
  let mut tokens: Vec<String> = vec![];
  let mut token: String = String::new();
  let mut pushed: bool = true;
  for c in input.chars()
  {
    if symbols.contains(&c)
    {
      if !pushed
      {
        tokens.push(token);
        token = String::new();
        pushed = true;
      }
      tokens.push(String::from(c));
    }
    else
    {
      pushed = false;
      token.push(c);
    }
  }
  if !pushed
  {
    tokens.push(token);
  }
  return tokens;
}


#[test]
fn test_lexer()
{
  use std::iter::zip;
  let symbols = ['+','-','*','/','=','(',')', '.'];
  let symbol_set: HashSet<char> = HashSet::from(symbols);
  let input: &str = "69+(420-9*13)=420";
  let split_input = lexer(&input, &symbol_set);
  let output = ["69", "+", "(", "420", "-", "9", "*", "13", ")", "=", "420"];
  for (x,y) in zip(split_input, output)
  {
    assert_eq!(x,y);
  }
}