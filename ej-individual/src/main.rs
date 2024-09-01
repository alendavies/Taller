mod clauses;
mod errors;
mod register;
mod table;

use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
    path::Path,
};

use clauses::{insert_sql::Insert, select_sql::Select};
use errors::CustomError;
use register::Register;
use table::Table;

fn parse(string: &str) -> Vec<String> {
    let mut index = 0;
    let mut tokens = Vec::new();
    let mut current = String::new();

    let string = string.replace(";", "");
    let length = string.len();

    let mut char = string.chars().nth(index).unwrap_or('0');

    while index < length {
        if char.is_alphabetic() || char == '_' {
            while (char.is_alphabetic() || char == '_') && index < length {
                current.push(char);
                index += 1;
                char = string.chars().nth(index).unwrap_or('0');
            }
            tokens.push(current);
            current = String::new();
        } else if char.is_numeric() {
            while char.is_numeric() && index < length {
                index += 1;
                char = string.chars().nth(index).unwrap_or('0');
            }

            tokens.push(current);
            current = String::new();
        } else {
            if char.is_whitespace() || char == ',' {
                index += 1;
                char = string.chars().nth(index).unwrap_or('0');
            } else if char == '\'' {
                index += 1;
                char = string.chars().nth(index).unwrap_or('0');

                while char != '\'' && index < length {
                    current.push(char);
                    index += 1;
                    char = string.chars().nth(index).unwrap_or('0');
                }

                tokens.push(current);
                current = String::new();
                index += 1;
                char = string.chars().nth(index).unwrap_or('0');
            } else if char == '(' {
                index += 1;
                char = string.chars().nth(index).unwrap_or('0');

                while char != ')' && index < length {
                    current.push(char);
                    index += 1;
                    char = string.chars().nth(index).unwrap_or('0');
                }

                tokens.push(current);
                current = String::new();
                index += 1;
                char = string.chars().nth(index).unwrap_or('0');
            } else {
                while !char.is_alphanumeric() && !char.is_whitespace() && index < length {
                    current.push(char);
                    index += 1;
                    char = string.chars().nth(index).unwrap_or('0');
                }
                tokens.push(current);
                current = String::new();
            }
        }
    }
    tokens.retain(|s| !s.is_empty());

    tokens
}

/* fn find_file_in_folder(folder_path: &str, file_name: &str) -> Option<String> {
    let path = Path::new(folder_path);

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if let Some(name) = entry_path.file_name() {
                    if name == file_name {
                        return Some(entry_path.to_string_lossy().into_owned());
                    }
                }
            }
        }
    }

    None
} */

fn exec_query(file_path: &str, query: &str, folder: &Path) -> Result<Vec<String>, CustomError> {
    let tokens = parse(query);
    let mut result = Table::new();
    let mut register = Register(HashMap::new());
    let mut result_csv = Vec::new();

    match tokens.first().ok_or(CustomError::Error)?.as_str() {
        "SELECT" => {
            let file = File::open(file_path).map_err(|e| CustomError::FileError)?;
            let reader = BufReader::new(file);
            let clause = Select::new_from_tokens(tokens);
            for (idx, line) in reader.lines().enumerate() {
                let line = line.map_err(|_| CustomError::ReaderError)?;
                if idx == 0 {
                    result.columns = line.split(',').map(|s| s.to_string()).collect();
                    continue;
                }
                register = clause.execute(line, &result.columns);

                if !register.0.is_empty() {
                    result.registers.push(register);
                }
            }

            if clause.orderby_clause.column != "" {
                let registers_ordered = clause.orderby_clause.execute(&mut result.registers);
                result.registers = registers_ordered.to_vec();
            }

            result_csv = serialize_result(result, clause.columns)?;
        }
        "INSERT" => {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(file_path)
                .map_err(|e| CustomError::Error)?;

            let clause = Insert::new_from_tokens(tokens);

            clause.execute(&mut file)?;
        }
        _ => todo!(),
    }

    Ok(result_csv)
}

fn serialize_result(table: Table, column_order: Vec<String>) -> Result<Vec<String>, CustomError> {
    let mut result: Vec<String> = Vec::new();

    result.push(column_order.join(","));

    for register in table.registers {
        let register_csv = register.to_csv(&column_order)?;
        result.push(register_csv);
    }

    Ok(result)
}

fn main() -> Result<(), CustomError> {
    let example = vec![
        "tabla.csv",
        "INSERT INTO ordenes (id, id_cliente, producto, cantidad) VALUES (111, 6, 'Laptop', 3);
",
    ];

    let folder_path = "./";

    let folder = Path::new(folder_path);

    let table = example[0];
    let query = example[1];

    let result = exec_query(table, query, folder)?;

    for line in result {
        println!("{}", line);
    }

    Ok(())
}
