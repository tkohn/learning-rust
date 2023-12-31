fn main() {
    //vector_example();
    //string_examples();
    hash_map_examples();
}

use std::collections::HashMap;
fn hash_map_examples() {
    println!("########### HashMap Examples ###########");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10); // i32 will be copied, String will be moved
    scores.entry(String::from("Yellow")).or_insert(50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    scores.entry(String::from("Blue")).or_insert(25); // a value exists, so it will not insert 25 for Key Blue
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.insert(String::from("Blue"), 40);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{field_name}{field_value}"); // value borrowed here after move

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
fn string_examples() {
    println!("########### String Examples ###########");
    let mut s = String::new();
    s.push_str("Hello World");

    let data = "initial contens";
    let s = data.to_string();

    let s = "initial contens".to_string();

    let s = String::from("initial contens");
    hello();

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    s1.push('1');
    println!("{s1}");

    let s1 = String::from("Hello ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2;
    // The + operator uses the add method, whose signature looks something like this:
    // fn add(self, s: &str) -> String
    // First, s2 has an &, meaning that we’re adding a reference of the second string to the first string. This is because of the s parameter in the add function: we can only add a &str to a String; we can’t add two String values together. But wait—the type of &s2 is &String, not &str, as specified in the second parameter to add. So why does Listing 8-18 compile?
    //
    // The reason we’re able to use &s2 in the call to add is
    // that the compiler can coerce the &String argument into a &str.
    // When we call the add method, Rust uses a deref coercion,
    // which here turns &s2 into &s2[..].

    // let s3 = s1 + &s2;
    // fn add(self, s: &str) -> String
    // 1. add takes ownership of s1,
    // 2. it appends a copy of the contents of s2 to s1,
    // 3. and then it returns back ownership of s1.

    println!("{s3}");
    //println!("{s1}"); // value borrowed here after move
    println!("{s2}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // the code generated by the format! macro uses references
    // so that this call doesn’t take ownership of any of its parameters.
    let s = format!("{s1}-{s2}-{s3}"); // format! -> macro
    println!("{s}");

    let s1 = String::from("hello");
    // let h = s1[0]; // -> `String` cannot be indexed by `{integer}`
    // see also:  bytes, scalar values, and grapheme clusters
    // and: Devanagari script is complex

    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }
}

fn hello() {
    let hellos = vec![
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("Hello"),
        String::from("שָׁלוֹם"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];

    for hello in hellos {
        println!("{}", hello);
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vector_example() {
    println!("########### Vector Examples ###########");
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

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
