fn first() {
    println!("Function ordering does not matter");
}
fn main() {
    println!("Hello, world!");
    another_function(-100);
    first();
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3; // <- statement
        let z = 1 + 1; // <- statement with the expression '1 + 1'
        x + 1; // return unit because of the ending ';',
               // which turns the expression to a statement
    };
    println!("The value of y is: {y:?}");

    let y = {
        let x = 3;
        x + 1 // expression, without ';' at end of line
    };
    println!("The value of y is: {y:?}");

    println!("The value of five() is: {}", five());
    println!("The value of plus_one(6) is: {}", plus_one(6));
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
fn five() -> i32 {
    5
}

// ordering of the function does not matter
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
