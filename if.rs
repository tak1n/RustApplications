fn main() {
  let x = 5;

  if x == 5 {
    println!("x is five!");
  } else if x == 6 {
    println!("x is six!");
  } else {
    println!("x is not five or six :(");
  }

  let x = 6;

  // this
  let y = if x == 5 {
    10
  } else {
    15
  }; // y: i32

  // is the same as
  // let y = if x == 5 { 10 } else { 15 }; // y: i32

  println!("{}", y);
}
