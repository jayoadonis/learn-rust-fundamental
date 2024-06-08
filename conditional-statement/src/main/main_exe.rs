

#![allow(unused)]
#![allow(dead_code)]

mod if_let;
mod let_match;

pub fn main() -> Result<(), usize> {
    println!("Fundamental Conditional statement.");
    dbg!(let_match::execute().unwrap_or_default());
    return Ok(());
}