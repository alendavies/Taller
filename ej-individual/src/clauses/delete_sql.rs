use super::where_sql::Where;
use crate::{
    errors::{CustomError, SqlError},
    register::Register,
    table::Table,
    utils::find_file_in_folder,
};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader},
};

use std::io::Write;

pub struct Delete {
    pub table_name: String,
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

        let table_name = tokens[i].to_string();
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
            table_name,
            where_clause,
        }
    }

    pub fn apply_to_table(&self, table: BufReader<File>) -> Result<Table, SqlError> {
        let mut result = Table::new();

        for (idx, line) in table.lines().enumerate() {
            let line = line.map_err(|_| SqlError::Error(CustomError::ReaderError))?;

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
        let mut temp_file =
            File::create(&temp_file_path).map_err(|_| SqlError::Error(CustomError::FileError))?;

        for line in csv {
            writeln!(temp_file, "{}", line)
                .map_err(|_| SqlError::Error(CustomError::WriteError))?;
        }

        let path = folder_path.to_string() + "/" + &self.table_name + ".csv";

        fs::rename(&temp_file_path, path).map_err(|_| SqlError::Error(CustomError::FileError))?;

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
        }

        result
    }
}
