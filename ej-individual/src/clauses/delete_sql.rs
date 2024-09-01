use super::where_sql::Where;
use crate::register::Register;
use std::collections::HashMap;

pub struct From(String);

pub struct Delete {
    pub from_clause: From,
    pub where_clause: Where,
}

impl Delete {
    pub fn new_from_tokens(tokens: Vec<String>) -> Self {
        if !tokens.contains(&String::from("DELETE")) || !tokens.contains(&String::from("FROM")) {
            println!("Clausula DELETE inválida");
        }

        let mut where_tokens: Vec<&str> = Vec::new();

        let mut i = 0;

        if tokens[i] != "DELETE" {
            println!("Clausula DELETE inválida");
        }

        i += 1;

        if tokens[i] != "FROM" {
            println!("Clausula DELETE inválida");
        }
        i += 1;

        let from = tokens[i].to_string();
        i += 1;

        if i < tokens.len() {
            if tokens[i] == "WHERE" {
                while i < tokens.len() {
                    where_tokens.push(tokens[i].as_str());
                    i += 1;
                }
            }
        }

        let where_clause = Where::new_from_tokens(where_tokens);

        Self {
            from_clause: From(from),
            where_clause,
        }
    }

    pub fn execute(&self, line: String, columns: &Vec<String>) -> Register {
        let atributes: Vec<String> = line.split(',').map(|s| s.to_string()).collect();

        let mut register = Register(HashMap::new());

        for (idx, col) in columns.iter().enumerate() {
            register
                .0
                .insert(col.to_string(), atributes[idx].to_string());
        }

        let mut result = Register(HashMap::new());

        if self.where_clause.column.len() > 0 {
            let op_result = self.where_clause.execute(&register.0);

            if op_result == false {
                for col in columns {
                    result.0.insert(
                        col.to_string(),
                        register.0.get(col).unwrap_or(&String::new()).to_string(),
                    );
                }
            }
        } else {
            return result;
        }

        result
    }
}
