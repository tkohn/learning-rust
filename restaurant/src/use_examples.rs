fn main() {

}

use std::fmt;
//use std::io; // see below

fn function1() {
    let mut r : fmt::Result = None;
}

fn function2() {
    let mut r : io::Result<()> = None;
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() {
    let mut r : Result = None;
}

fn function4() {
    let mut r : IoResult = None;
}

// use std::cmp::Ordering;
// use std::io;
// combine above two lines into one line ->
// use std::{cmp::Ordering, io}; // alternative usage of use

//use std::io;
//use std::io::Write;
// combine above two lines into one line ->
use std::io::{self, Write};

// Glob Operator
use std::collections::*;
