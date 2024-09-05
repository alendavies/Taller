use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{errors::SqlError, register::Register, table::Table, utils::find_file_in_folder};

use super::{orderby_sql::OrderBy, where_sql::Where};

pub struct Select {
    pub table_name: String,
    pub columns: Vec<String>,
    pub where_clause: Where,
    pub orderby_clause: OrderBy,
}

impl Select {
    pub fn new_from_tokens(tokens: Vec<String>) -> Result<Self, SqlError> {
        if !tokens.contains(&String::from("WHERE")) || !tokens.contains(&String::from("SELECT")) {
            println!("Clausula SELECT inválida");
        }

        let mut columns: Vec<&str> = Vec::new();
        let mut where_tokens: Vec<&str> = Vec::new();
        let mut orderby_tokens: Vec<&str> = Vec::new();

        let mut i = 0;
        let mut table_name = String::new();

        if tokens[i] == "SELECT" {
            i += 1;
            while tokens[i] != "FROM" {
                columns.push(tokens[i].as_str());
                i += 1;
            }
        }
        if tokens[i] == "FROM" {
            i += 1;
            table_name = tokens[i].to_string();
        }
        i += 1;

        if tokens[i] == "WHERE" {
            where_tokens.push(tokens[i].as_str());
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
            orderby_tokens.push(tokens[i].as_str());
            i += 1;
            orderby_tokens.push(tokens[i].as_str());
            i += 1;
            while i < tokens.len() {
                orderby_tokens.push(tokens[i].as_str());
                i += 1;
            }
        }

        let where_clause = Where::new_from_tokens(where_tokens)?;
        let orderby_clause = OrderBy::new_from_tokens(orderby_tokens);

        Ok(Self {
            table_name,
            columns: columns.iter().map(|c| c.to_string()).collect(),
            where_clause,
            orderby_clause,
        })
    }

    pub fn apply_to_table(&self, table: BufReader<File>) -> Result<Table, SqlError> {
        let mut result = Table::new();

        for (idx, line) in table.lines().enumerate() {
            let line = line.map_err(|_| SqlError::Error)?;
            if idx == 0 {
                result.columns = line.split(',').map(|s| s.to_string()).collect();
                continue;
            }
            let register = self.execute(line, &result.columns)?;

            if !register.0.is_empty() {
                result.registers.push(register);
            }
        }

        if self.orderby_clause.column != "" {
            let registers_ordered = self.orderby_clause.execute(&mut result.registers);
            result.registers = registers_ordered.to_vec();
        }

        Ok(result)
    }

    pub fn execute(&self, line: String, columns: &Vec<String>) -> Result<Register, SqlError> {
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

        match op_result {
            Ok(value) => {
                if value == true {
                    for col in col_selected {
                        result.0.insert(
                            col.to_string(),
                            register.get(&col).unwrap_or(&String::new()).to_string(),
                        );
                    }
                }
                return Ok(result);
            }
            Err(_) => return Err(SqlError::Error),
        }
    }

    pub fn open_table(&self, folder_path: &str) -> Result<BufReader<File>, SqlError> {
        let table_name = self.table_name.to_string() + ".csv";
        if !find_file_in_folder(folder_path, &table_name) {
            return Err(SqlError::InvalidTable);
        }
        let table_path = folder_path.to_string() + "/" + &table_name;
        let file = File::open(&table_path).map_err(|_| SqlError::InvalidTable)?;

        let reader = BufReader::new(file);

        Ok(reader)
    }
}
