use crate::register::Register;

#[derive(Debug)]
pub struct Table {
    pub columns: Vec<String>,
    pub registers: Vec<Register>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            registers: Vec::new(),
        }
    }
}
