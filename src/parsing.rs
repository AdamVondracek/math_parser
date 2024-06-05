fn math_parser(token: &str)
{
  match token
  {
    "+" => println!("Plus"),
    "-" => println!("Minus"),
    _ => println!("Something else")
  }
}