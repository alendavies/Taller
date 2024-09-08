use super::into_sql::Into;
use crate::errors::SqlError;
use crate::utils::{find_file_in_folder, is_insert, is_values};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};

pub struct Insert {
    pub values: Vec<String>,
    pub into_clause: Into,
}

impl Insert {
    pub fn new_from_tokens(tokens: Vec<String>) -> Result<Self, SqlError> {
        if tokens.len() < 6 {
            return Err(SqlError::InvalidSyntax);
        }
        let mut into_tokens: Vec<&str> = Vec::new();
        let mut values: Vec<String> = Vec::new();

        let mut i = 0;

        if is_insert(&tokens[i]) {
            i += 1;
            while !is_values(&tokens[i]) && i < tokens.len() {
                into_tokens.push(tokens[i].as_str());
                i += 1;
            }
        }
        if is_values(&tokens[i]) {
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

        if into_tokens.is_empty() || values.is_empty() {
            return Err(SqlError::InvalidSyntax);
        }

        let into_clause = Into::new_from_tokens(into_tokens)?;

        Ok(Self {
            values,
            into_clause,
        })
    }

    pub fn apply_to_table(&mut self, file: &mut File) -> Result<(), SqlError> {
        let mut reader = BufReader::new(file.by_ref());

        let mut first_line = String::new();

        reader
            .read_line(&mut first_line)
            .map_err(|_| SqlError::InvalidTable)?;

        let columns: Vec<String> = first_line
            .trim()
            .split(',')
            .map(|col| col.to_string())
            .collect();

        self.reorder_values(columns);

        println!("{:?}", self.into_clause.columns);
        println!("{:?}", self.values);

        let line = self.values.join(",");

        file.seek(SeekFrom::End(0)).map_err(|_| SqlError::Error)?;

        writeln!(file, "{}", line).map_err(|_| SqlError::Error)?;

        Ok(())
    }

    pub fn reorder_values(&mut self, columns: Vec<String>) {
        let mut reordered_values: Vec<&str> = Vec::new();
        let mut reordered_cols: Vec<&str> = Vec::new();

        for col in &columns {
            if self.into_clause.columns.contains(col) {
                if let Some(index) = self.into_clause.columns.iter().position(|x| x == col) {
                    reordered_values.push(self.values[index].as_str());
                }

                reordered_cols.push(col);
            } else {
                reordered_values.push("");
                reordered_cols.push(col);
            }
        }

        self.into_clause.columns = reordered_cols.iter().map(|c| c.to_string()).collect();
        self.values = reordered_values.iter().map(|c| c.to_string()).collect();
    }

    pub fn open_table(&self, folder_path: &str) -> Result<File, SqlError> {
        let table_name = self.into_clause.table_name.to_string() + ".csv";
        if !find_file_in_folder(folder_path, &table_name) {
            return Err(SqlError::InvalidTable);
        }
        let table_path = folder_path.to_string() + "/" + &table_name;

        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(&table_path)
            .map_err(|_| SqlError::InvalidTable)?;

        Ok(file)
    }
}
