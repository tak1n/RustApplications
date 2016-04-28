fn main() {
  let x: i32 = 17;
  {
      let y: i32 = 3;
      println!("The value of x is {} and value of y is {}", x, y);
  }
  let x = 5; // rebinding & type inference (i32)
  println!("The value of x is {}", x); // This won't work
}
