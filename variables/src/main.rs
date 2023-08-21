fn main() {
    // mutable vs immutable
    let mut x = 5; // mut is needed
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadowing
    let y = 5;
    println!("let y = 5: {y}");
    let y = y + 1; // shadowing above variable
    println!("let y = y + 1: {y}");

    {
        // creates a new scope
        let y = y * 2;
        println!("let y = y * 2: {y} (scope)");
    }

    println!("y: {y} (outside above scope)");

    let spaces = "   ";
    let spaces = spaces.len(); // shadowing can change the type
    println!("number of spaces: {spaces}");

    // it does not work with mutable:
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // because it re-assign the variable and not shadowing it.
    // For that reason it can not change the type
}
