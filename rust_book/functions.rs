fn main() {
  print_number(5);
  print_sum(5, 6);

  println!("{}", add_one(5));
  println!("{}", early_return(5));

  // function pointers
  // or variable binding which points to a function
  let f: fn(i32, i32) -> () = print_sum;
  f(6, 6);

  crash(6);
}

fn print_number(x: i32) {
  println!("x is {}", x);
}

fn print_sum(x: i32, y: i32) {
  println!("sum is: {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn early_return(x: i32) -> i32 {
  if x == 5 {
    return x;
  }

  x + 1
}

// diverging functions have return type "!" (they never return)
// use RUST_BACKTRACE env var for more information
fn crash(x: i32) -> ! {
  panic!("{} asdf", x);
}
