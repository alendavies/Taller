use std::collections::HashMap;

use crate::register::Register;

use super::{orderby_sql::OrderBy, where_sql::Where};

pub struct Select {
    pub columns: Vec<String>,
    pub where_clause: Where,
    pub orderby_clause: OrderBy,
}

impl Select {
    pub fn new_from_tokens(tokens: Vec<String>) -> Self {
        if !tokens.contains(&String::from("WHERE")) || !tokens.contains(&String::from("SELECT")) {
            println!("Clausula SELECT inválida");
        }

        let mut columns: Vec<&str> = Vec::new();
        let mut where_tokens: Vec<&str> = Vec::new();
        let mut orderby_tokens: Vec<&str> = Vec::new();

        let mut i = 0;

        if tokens[i] == "SELECT" {
            i += 1;
            while tokens[i] != "FROM" {
                columns.push(tokens[i].as_str());
                i += 1;
            }
            i += 2;
        }
        if tokens[i] == "WHERE" {
            i += 1;
            if tokens.contains(&String::from("ORDER")) {
                while tokens[i] != "ORDER" {
                    where_tokens.push(tokens[i].as_str());
                    i += 1;
                }
            } else {
                while i < tokens.len() {
                    where_tokens.push(tokens[i].as_str());
                    i += 1;
                }
            }
        }
        if i < tokens.len() && tokens[i] == "ORDER" && tokens[i + 1] == "BY" {
            i += 2;
            while i < tokens.len() {
                orderby_tokens.push(tokens[i].as_str());
                i += 1;
            }
        }

        let where_clause = Where::new_from_tokens(where_tokens);
        let orderby_clause = OrderBy::new_from_tokens(orderby_tokens);

        Self {
            columns: columns.iter().map(|c| c.to_string()).collect(),
            where_clause,
            orderby_clause,
        }
    }

    pub fn execute(&self, line: String, columns: &Vec<String>) -> Register {
        let atributes: Vec<String> = line.split(',').map(|s| s.to_string()).collect();

        let mut register = Register(HashMap::new()).0;

        for (idx, col) in columns.iter().enumerate() {
            register.insert(col.to_string(), atributes[idx].to_string());
        }

        let mut col_selected = Vec::new();
        if self.columns[0] == "*" {
            for col in columns {
                col_selected.push(col.to_string());
            }
        } else {
            for col in &self.columns {
                col_selected.push(col.to_string());
            }
        }

        let mut result = Register(HashMap::new());
        let op_result = self.where_clause.execute(&register);

        if op_result == true {
            for col in col_selected {
                result.0.insert(
                    col.to_string(),
                    register.get(&col).unwrap_or(&String::new()).to_string(),
                );
            }
        }

        result
    }
}
