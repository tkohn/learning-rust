use crate::List::{Cons, Nil};
use crate::ListRc::{ConsRc, NilRc};
use std::ops::Deref;
use std::rc::Rc;

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
    println!("{} {}", x, *n);

    // CustomSmartPointer Examples
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("Other stuff"),
    };
    // c.drop(); // can not be used directly, it is called implicit
    drop(c); // std::mem::drop
    println!("CustomSmartPointer created.");

    // Variables are dropped in the reverse order of their creation,
    // so 'd' was dropped before 'c'

    let x = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let y = Cons(3, Box::new(x));
    // let z = Cons(4, Box::new(x)); // x -> use of moved value

    println!("######## Rc Examples");
    let a = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(NilRc)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = ConsRc(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = ConsRc(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let x = 5;
    //let y = &mut x; // cannot borrow `x` as mutable, as it is not declared as mutable
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum ListRc {
    ConsRc(i32, Rc<ListRc>),
    NilRc,
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
