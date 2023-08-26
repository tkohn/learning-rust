fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    println!("{}", v[0]); // -> value
    println!("{}", v[1]);
    println!("{:?}", v.get(2)); // -> Option<i32>
    println!("{:?}", v.get(3));

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);

    match third {
        None => println!("There is no third element."),
        Some(third) => println!("The third element is {third}"),
    }

    for n_ref in &mut v {
        *n_ref += 10;
    }

    for n_ref in v {
        println!("{n_ref}");
    }

    let v = vec![1, 2, 3]; // vec! -> macro to create a new vector

    let v: Vec<i32> = vec![];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn test_spreadsheet_cell() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
