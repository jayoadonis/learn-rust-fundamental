use std::cell::Cell;

#[derive(Debug)]
pub struct EntityError<'a> {
    pub code: Cell<usize>,
    pub description: Cell<&'a str>
}

impl<'a> EntityError<'a> {
    pub fn new( code: usize, description: &'a str ) -> Self {
        return Self {
            code: Cell::new(code),
            description: Cell::new(description)
        };
    }
}