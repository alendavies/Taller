use super::where_sql::Where;
use crate::register::Register;
use std::collections::HashMap;

pub struct Set(Vec<(String, String)>);

impl Set {
    fn new_from_tokens(tokens: Vec<String>) -> Self {
        let mut set = Vec::new();

        if tokens.len() < 1 {
            return Self(set);
        }

        let mut i = 0;

        if tokens[i] != "SET" || !tokens.contains(&"=".to_string()) {
            println!("Error en clausula SET");
        }

        i += 1;

        while i < tokens.len() {
            if tokens[i] == "=" {
                set.push((tokens[i - 1].to_string(), tokens[i + 1].to_string()));
            }
            i += 1;
        }

        Self(set)
    }
}

pub struct Update {
    pub table_name: String,
    pub where_clause: Where,
    pub set_clause: Set,
}

impl Update {
    pub fn new_from_tokens(tokens: Vec<String>) -> Self {
        if !tokens.contains(&String::from("UPDATE")) || !tokens.contains(&String::from("SET")) {
            println!("Clausula UPDATE inválida");
        }

        let mut where_tokens: Vec<&str> = Vec::new();
        let mut set_tokens = Vec::new();

        let mut i = 0;

        if tokens[i] != "UPDATE" {
            println!("Clausula UPDATE inválida");
        }
        i += 1;
        let table = tokens[i].to_string();

        i += 1;
        if tokens[i] != "SET" {
            println!("Clausula UPDATE inválida");
        }

        while tokens[i] != "WHERE" && i < tokens.len() {
            set_tokens.push(tokens[i].to_string());
            i += 1;
        }

        if i < tokens.len() && tokens[i] == "WHERE" {
            while i < tokens.len() {
                where_tokens.push(tokens[i].as_str());
                i += 1;
            }
        }

        let where_clause = Where::new_from_tokens(where_tokens);
        let set_clause = Set::new_from_tokens(set_tokens);

        Self {
            table_name: table,
            where_clause,
            set_clause,
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

        if self.where_clause.column.len() > 0 {
            let op_result = self.where_clause.execute(&register.0);

            if op_result == true {
                for (col, val) in &self.set_clause.0 {
                    register.0.insert(col.to_string(), val.to_string());
                }
            }
        } else {
            for (col, val) in &self.set_clause.0 {
                register.0.insert(col.to_string(), val.to_string());
            }
        }

        register
    }
}
