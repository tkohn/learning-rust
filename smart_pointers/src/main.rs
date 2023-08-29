use crate::List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    examples();

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // deref coercion improves readability
    hello(&(*m)[..]);
    // (*m) MyBox<String> -> String
    // &[..] String -> &str ([..] <- String slice from index 0 and last index)

    // Rust does deref coercion when it finds types and trait implementations in three cases:
    // * From &T to &U when T: Deref<Target=U>
    // * From &mut T to &mut U when T: DerefMut<Target=U>
    // * From &mut T to &U when T: Deref<Target=U>

    let n = AccessLogger(-1);
    let x = *n + 1;
    let n2 = n;
    println!("{} {}", x, *n)
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy)]
struct AccessLogger(i32);
impl Deref for AccessLogger {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        println!("deref");
        &self.0
    }
}

fn examples() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    let x = 5;
    let y = &x;
    let z = x + y;
    assert_eq!(5, x);
    //assert_eq!(5, y); // no implementation for `{integer} == &{integer}`
    assert_eq!(5, *y);
    assert_eq!(10, z);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
