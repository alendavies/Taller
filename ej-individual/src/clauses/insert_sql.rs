use std::fs::{File, OpenOptions};
use std::io::Write;

use crate::errors::{CustomError, SqlError};
use crate::utils::find_file_in_folder;
pub struct Into {
    table_name: String,
    columns: Vec<String>,
}

impl Into {
    pub fn new_from_tokens(tokens: Vec<&str>) -> Self {
        let mut i = 0;
        let mut table_name = String::new();
        let mut columns: Vec<String> = Vec::new();

        if tokens[i] == "INTO" {
            i += 1;
            table_name = tokens[i].to_string();
            i += 1;

            let cols: Vec<String> = tokens[i].split(",").map(|c| c.trim().to_string()).collect();

            for col in cols {
                columns.push(col);
            }
        } else {
            println!("Error al crear Into");
        }

        Self {
            table_name,
            columns,
        }
    }
}

pub struct Insert {
    pub values: Vec<String>,
    pub into_clause: Into,
}

impl Insert {
    pub fn new_from_tokens(tokens: Vec<String>) -> Self {
        if !tokens.contains(&String::from("VALUES")) || !tokens.contains(&String::from("INSERT")) {
            println!("Clausula INSERT inválida");
        }

        let mut into_tokens: Vec<&str> = Vec::new();
        let mut values: Vec<String> = Vec::new();

        let mut i = 0;

        if tokens[i] == "INSERT" {
            i += 1;
            while tokens[i] != "VALUES" {
                into_tokens.push(tokens[i].as_str());
                i += 1;
            }
        }
        if tokens[i] == "VALUES" {
            i += 1;

            let vals: Vec<String> = tokens[i]
                .replace("\'", "")
                .split(",")
                .map(|c| c.trim().to_string())
                .collect();

            for val in vals {
                values.push(val);
            }
        }

        let into_clause = Into::new_from_tokens(into_tokens);

        Self {
            values,
            into_clause,
        }
    }

    pub fn open_table(&self, folder_path: &str) -> Result<File, SqlError> {
        let table_name = self.into_clause.table_name.to_string() + ".csv";
        if !find_file_in_folder(folder_path, &table_name) {
            return Err(SqlError::InvalidTable);
        }
        let table_path = folder_path.to_string() + "/" + &table_name;
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&table_path)
            .map_err(|_| SqlError::InvalidTable)?;

        Ok(file)
    }

    pub fn apply_to_table(&self, file: &mut File) -> Result<(), SqlError> {
        let line = self.values.join(",");
        writeln!(file, "{}", line).map_err(|_| SqlError::Error(CustomError::WriteError))?;

        Ok(())
    }
}
