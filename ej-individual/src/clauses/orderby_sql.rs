use crate::register::Register;

pub struct OrderBy {
    pub column: String,
    pub order: String,
}

impl OrderBy {
    pub fn new_from_tokens(tokens: Vec<&str>) -> Self {
        let mut column = String::new();
        let mut order = String::new();

        if tokens.len() < 1 {
            return Self { column, order };
        }

        for token in tokens {
            if token.chars().all(|c| c.is_alphabetic()) {
                if token.chars().all(|c| c.is_lowercase()) {
                    column = token.to_string();
                } else if token.chars().all(|c| c.is_uppercase()) {
                    order = token.to_string();
                }
            }
        }
        Self { column, order }
    }

    pub fn execute<'a>(&self, registers: &'a mut Vec<Register>) -> &'a Vec<Register> {
        registers.sort_by(|a, b| {
            let default = String::new();
            let val_a = a.0.get(&self.column).unwrap_or(&default);
            let val_b = b.0.get(&self.column).unwrap_or(&default);
            if self.order == "DESC" {
                val_b.cmp(val_a)
            } else {
                val_a.cmp(val_b)
            }
        });

        registers
    }
}
