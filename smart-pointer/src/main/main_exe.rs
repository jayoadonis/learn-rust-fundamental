
#![allow(unused)]
#![allow(dead_code)]

mod model;
mod box_smart_pointer;

pub fn main() -> Result<(), usize> {
    println!("Smart pointer.");
    crate::box_smart_pointer::execute();
    return Ok(());
}