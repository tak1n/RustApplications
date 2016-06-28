fn main() {
    // Loops forever
    // loop {
    //   println!("Loop forever!");
    // }


    let mut x = 5; // mut x: i32
    let mut done = false; // mut done: bool

    while !done {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }

    println!("");

    for x in 0..10 {
        println!("{}", x); // x: i32
    }

    println!("");

    for (i,j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }

    println!("");

    let lines = "hello\nworld".lines();

    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }

    println!("");

    let mut x = 5;

    loop {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 { break; }
    }

    println!("");

    for x in 0..10 {
        if x % 2 == 0 { continue; }

        println!("{}", x);
    }
}
