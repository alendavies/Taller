use std::cmp::Ordering;

use crate::{
    errors::SqlError,
    register::Register,
    utils::{is_by, is_order},
};

#[derive(Debug, PartialEq)]
pub struct OrderBy {
    pub columns: Vec<String>,
    pub order: String,
}

impl OrderBy {
    pub fn new_from_tokens(tokens: Vec<&str>) -> Result<Self, SqlError> {
        if tokens.len() < 3 {
            return Err(SqlError::InvalidSyntax);
        }

        let mut columns = Vec::new();
        let mut order = String::new();
        let mut i = 0;

        if !is_order(tokens[i]) && !is_by(tokens[i + 1]) {
            return Err(SqlError::InvalidSyntax);
        }

        i += 2;

        while i < tokens.len() && tokens[i] != "DESC" && tokens[i] != "ASC" {
            columns.push(tokens[i].to_string());
            i += 1;
        }

        if i < tokens.len() {
            if tokens[i] == "DESC" || tokens[i] == "ASC" {
                order = tokens[i].to_string();
            }
        }

        Ok(Self { columns, order })
    }

    pub fn execute<'a>(&self, registers: &'a mut Vec<Register>) -> &'a Vec<Register> {
        registers.sort_by(|a, b| {
            let mut result = Ordering::Equal;
            for column in &self.columns {
                if let Some(val_a) = a.0.get(column) {
                    if let Some(val_b) = b.0.get(column) {
                        result = if self.order == "DESC" {
                            val_b.cmp(val_a)
                        } else {
                            val_a.cmp(val_b)
                        };
                        if result != Ordering::Equal {
                            break;
                        }
                    }
                }
            }
            result
        });
        registers
    }
}
