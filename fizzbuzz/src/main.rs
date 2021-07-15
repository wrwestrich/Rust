fn main() {
  // Old, C-like way
  // for i in 1..101 {
  //   if i % 3 == 0 && i % 5 == 0 {
  //     println!("fizzbuzz");
  //   } else if i % 5 == 0 {
  //     println!("buzz");
  //   } else if i % 3 == 0 {
  //     println!("fizz");
  //   } else {
  //     println!("{}", i);
  //   }
  // }

  // New Rust way using match
  for i in 1..101 {
    match (i % 3, i % 5) {
      (0, 0) => println!("fizzbuzz"),
      (0, _) => println!("fizz"),
      (_, 0) => println!("buzz"),
      _ => println!("{}", i),
    }
  }
}
