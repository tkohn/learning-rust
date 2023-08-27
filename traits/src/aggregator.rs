use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// same like above
pub fn notify_1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {} // arguments can have different type, they only need the same trait Summary
// pub fn notify<T: Summary>(item1: &T, item2: &T) {} // arguments have to be from the same type T and implemented the trait Summary

// Specifying Multiple Trait Bounds with the + Syntax
// pub fn notify(item: &(impl Summary + Display)) {}
// pub fn notify<T: Summary + Display>(item: &T) {}

// Clearer Trait Bounds with where Clauses
fn some_function1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    8
}
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    8
}
