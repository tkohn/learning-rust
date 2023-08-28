use std::thread;

fn main() {
    //closure_examples();
    let v1: Vec<i32> = vec![1, 2, 3];
    let x = v1.iter().map(|x| x + 1); // warning: unused `Map` that must be used
                                      //   = note: iterators are lazy and do nothing unless consumed
                                      //   = note: `#[warn(unused_must_use)]` on by default
    let v2: Vec<_> = x.collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

fn make_a_cloner(s_ref: &str) -> impl Fn() -> String + '_ {
    // We can remove the <'a> generic so long as we keep an indicator
    // that the returned closure depends on some lifetime, with + '_ on the return type
    move || s_ref.to_string()
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

fn closure_examples() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveawy2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveawy2
    );

    let add_one = |x| x + 1;

    println!("{:?}", add_one(5));

    let f = |_| (); // sometimes called the "toilet closure"
    let s = String::from("Hello");
    f(s);
    // The toilet closure is similar to std::mem::drop,
    // i.e. a function that moves an argument and causes it to be dropped.
    // println!("{s}");

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    // println!("Before defining closure: {:?}", list); // cannot borrow list
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list)) // move is needed here to be sure that the reference to list lives long enough
        .join()
        .unwrap();

    let s_own = String::from("Hello world");
    let cloner = make_a_cloner(&s_own);
    // drop(s_own); // cannot move out of `s_own` because it is borrowed
    cloner();
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
