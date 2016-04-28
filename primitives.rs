fn main() {
  // char
  let x: char = 'w';
  println!("{}", x);

  // bool
  let y: bool = false;
  println!("{}", y);

  // numeric types
  let x = 42; // x has type i32
  println!("{}", x);

  let y = 1.0; // y has type f64
  println!("{}", y);

  // all elements of an array must be of the same type
  let a = [1, 2, 3]; // a: [i32; 3]
  println!("{}", a[0]);
  let slice = &a[1..2];
  println!("{}", slice[0]);

  // tuples
  let mut x = (1, 2); // x: (i32, i32)
  let y = (2, 3); // y: (i32, i32)

  // You can assign one tuple into another if they have same arity and contained types
  x = y;

  // you can assign elements in a tuple through a destructuring let
  let (x, y) = x;

  println!("first element of tuple is: {}", x);
  println!("second element of tuple is: {}", y);

  // you can also access elements of a tuple through tuple indexes
  let tuple = (3, 5);
  println!("{}", tuple.0);

  // Functions also have a type
  let x: fn(i32) -> i32 = foo;
  println!("{}", x(5));
}

fn foo(x: i32) -> i32 { x + 1 }

