fn main() {
  let n = 12345678;

  let desc = if is_even(n) {
    "even"
  } else {
    "odd"
  };

  println!("{}", desc);
}

fn is_even(n: i32) -> bool {
  n % 2 == 0
}
