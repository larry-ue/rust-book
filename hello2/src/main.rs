fn main() {
  let a = 2;
  let b = 5;

  let res = add_with_lifetimes( &a, &b );
  println!("{}", res);
}

fn add_with_lifetimes<'a, 'b>( i: &'a i32, j: &'b i32) -> i32 {
  *i + *j
}
