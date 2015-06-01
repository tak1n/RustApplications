struct Person {
    name: String,
}

// pass by reference
fn f(p: &mut Person) {
    let mut s = "Sarah".to_string();
    s.push_str(p.name.as_ref());

    p.name = s;
}

// pass by value -> this will fail in rust because a struct has no copy constructor per default
// (implements Copy trait)
fn g(p: Person) -> Person {
    let mut s = "Sarah".to_string();
    s.push_str(p.name.as_ref());

    p.name = s;

    return p;
}

fn main() {

    let mut x = Person { name: "John".to_string() };
    f(&mut x);
    println!("{}", x.name);

    let mut y = Person { name: "John".to_string() };
    g(y);
    println!("{}", y.name);
}
