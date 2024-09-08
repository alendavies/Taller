use super::where_sql::Where;
use crate::{errors::SqlError, register::Register, table::Table, utils::find_file_in_folder};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader},
};

use std::io::Write;

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
    pub fn new_from_tokens(tokens: Vec<String>) -> Result<Self, SqlError> {
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

        let where_clause = Where::new_from_tokens(where_tokens)?;
        let set_clause = Set::new_from_tokens(set_tokens);

        Ok(Self {
            table_name: table,
            where_clause,
            set_clause,
        })
    }

    pub fn execute(&self, line: String, columns: &Vec<String>) -> Register {
        let atributes: Vec<String> = line.split(',').map(|s| s.to_string()).collect();

        let mut register = Register(HashMap::new());

        for (idx, col) in columns.iter().enumerate() {
            register
                .0
                .insert(col.to_string(), atributes[idx].to_string());
        }

        let op_result = self.where_clause.execute(&register);
        match op_result {
            Ok(result) => {
                if result == true {
                    for (col, val) in &self.set_clause.0 {
                        register.0.insert(col.to_string(), val.to_string());
                    }
                }
            }
            Err(_) => {
                for (col, val) in &self.set_clause.0 {
                    register.0.insert(col.to_string(), val.to_string());
                }
            }
        }

        register
    }

    pub fn apply_to_table(&self, table: BufReader<File>) -> Result<Table, SqlError> {
        let mut result = Table::new();

        for (idx, line) in table.lines().enumerate() {
            let line = line.map_err(|_| SqlError::Error)?;
            if idx == 0 {
                result.columns = line.split(',').map(|s| s.to_string()).collect();
                continue;
            }
            let register = self.execute(line, &result.columns);

            if !register.0.is_empty() {
                result.registers.push(register);
            }
        }
        Ok(result)
    }

    pub fn write_table(&self, csv: Vec<String>, folder_path: &str) -> Result<(), SqlError> {
        let temp_file_path = folder_path.to_string() + "/" + "temp.csv";
        let mut temp_file = File::create(&temp_file_path).map_err(|_| SqlError::Error)?;
        for line in csv {
            writeln!(temp_file, "{}", line).map_err(|_| SqlError::Error)?;
        }
        let path = folder_path.to_string() + "/" + &self.table_name + ".csv";
        fs::rename(&temp_file_path, path).map_err(|_| SqlError::Error)?;

        Ok(())
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
