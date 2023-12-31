use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // let v = vec![1, 2, 3];
    //
    // let handle = thread::spawn(|| { // closure may outlive the current function, but it borrows `v`, which is owned by the current function
    //     println!("Here's a vector: {:?}", v);
    // });
    //
    // handle.join().unwrap();

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // closure may outlive the current function, but it borrows `v`, which is owned by the current function
        println!("Here's a vector: {:?}", v);
    });
    // drop(v); // use of moved value: `v`
    handle.join().unwrap();
    //
    //

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let val = String::from("test");
        tx.send(val).unwrap();
        // println!("val is {}", val); // borrow of moved value: `val`

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
