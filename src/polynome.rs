use std::ops::*;

struct Polynome<T>
{
  variable_name: String,
  coeff: Vec<T>
}

impl<T> Polynome<T>
where
  T: std::fmt::Display,
  T: Add,
  T: AddAssign,
  T: Div,
  T: Mul<Output = T>,
  T: MulAssign,
  T: Copy,
  T: Clone,
{
  fn new(name: &str, arr: Vec<T>) -> Polynome<T>
  {
    return Polynome{
      variable_name: String::from(name),
      coeff: arr.clone()
    };
  }

  fn to_string(&self) -> String
  {
    return self.coeff
      .iter()
      .enumerate()
      .rev()
      .map(|(i, x)| format!("{x}{}^{i}", self.variable_name))
      .collect::<Vec<_>>()
      .join(" + ");
  }

  fn eval(&self, x: T) -> T
  {
    let mut exp: T = x;
    let mut sum: T = self.coeff[0];
    for i in 1..self.coeff.len()
    {
      sum += exp * self.coeff[i];
      exp *= x;
    }
    return sum;
  }
}


#[test]
fn test_print_polynome()
{
  let poly = Polynome::<i32>::new("x", vec![1,2,1]);
  println!("{} = {}",poly.to_string(), poly.eval(-1));
}