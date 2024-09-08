use std::{fs, path::Path};

use crate::{errors::SqlError, table::Table};

pub fn find_file_in_folder(folder_path: &str, file_name: &str) -> bool {
    let path = Path::new(folder_path);
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() && entry.file_name() == file_name {
                    return true;
                }
            }
        }
    }
    false
}

pub fn table_to_csv(table: &Table, column_order: &Vec<String>) -> Result<Vec<String>, SqlError> {
    let mut result: Vec<String> = Vec::new();

    result.push(column_order.join(","));

    for register in &table.registers {
        let register_csv = register.to_csv(&column_order)?;
        result.push(register_csv);
    }

    Ok(result)
}

pub fn is_number(token: &str) -> bool {
    token.parse::<i32>().is_ok()
}

pub fn is_and(token: &str) -> bool {
    token == "AND"
}

pub fn is_or(token: &str) -> bool {
    token == "OR"
}

pub fn is_not(token: &str) -> bool {
    token == "NOT"
}

pub fn is_left_paren(token: &str) -> bool {
    token == "("
}

pub fn is_right_paren(token: &str) -> bool {
    token == ")"
}

pub fn is_where(token: &str) -> bool {
    token == "WHERE"
}

pub fn is_select(token: &str) -> bool {
    token == "SELECT"
}

pub fn is_update(token: &str) -> bool {
    token == "UPDATE"
}

pub fn is_insert(token: &str) -> bool {
    token == "INSERT"
}

pub fn is_into(token: &str) -> bool {
    token == "INTO"
}

pub fn is_from(token: &str) -> bool {
    token == "FROM"
}

pub fn is_order(token1: &str) -> bool {
    token1 == "ORDER"
}
pub fn is_by(token1: &str) -> bool {
    token1 == "BY"
}

pub fn is_delete(token: &str) -> bool {
    token == "DELETE"
}

pub fn is_set(token: &str) -> bool {
    token == "SET"
}

pub fn is_values(token: &str) -> bool {
    token == "VALUES"
}
