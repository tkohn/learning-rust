type Kilometers = i32;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    let f: Thunk = Box::new(|| println!("hi"));
}

fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    // --snip--
}

type Thunk = Box<dyn Fn() + Send + 'static>; // type alias to reduce repetitive long types

fn takes_long_type2(f: Thunk) {
    // --snip--
}

// Rust has a special type named ! that’s known in type theory lingo as the empty type
// because it has no values. We prefer to call it the never type
// because it stands in the place of the return type when a function will never return.
// Here is an example:

fn bar() -> ! {
    panic!("Blub");
    // continue is also of type !
}
