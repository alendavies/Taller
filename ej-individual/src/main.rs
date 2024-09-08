mod clauses;
mod errors;
mod register;
mod table;
mod utils;

use clauses::{delete_sql::Delete, insert_sql::Insert, select_sql::Select, update_sql::Update};
use errors::SqlError;
use std::env;
use utils::table_to_csv;

fn tokens_from_query(string: &str) -> Vec<String> {
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
                current.push(char);
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

fn exec_query(folder_path: &str, query: &str) -> Result<Vec<String>, SqlError> {
    let tokens = tokens_from_query(query);
    let mut result_csv = Vec::new();

    match tokens.first().ok_or(SqlError::InvalidSyntax)?.as_str() {
        "SELECT" => {
            let clause = Select::new_from_tokens(tokens)?;
            let table = clause.open_table(folder_path)?;

            let result = clause.apply_to_table(table)?;
            if clause.columns[0] == "*" {
                result_csv = table_to_csv(&result, &result.columns)?;
            } else {
                result_csv = table_to_csv(&result, &clause.columns)?;
            }
        }
        "INSERT" => {
            let mut clause = Insert::new_from_tokens(tokens)?;
            let mut file = clause.open_table(folder_path)?;

            clause.apply_to_table(&mut file)?;
        }
        "DELETE" => {
            let clause = Delete::new_from_tokens(tokens)?;
            let table = clause.open_table(folder_path)?;

            let result = clause.apply_to_table(table)?;

            let csv = table_to_csv(&result, &result.columns)?;

            clause.write_table(csv, folder_path)?;
        }
        "UPDATE" => {
            let clause = Update::new_from_tokens(tokens)?;
            let table = clause.open_table(folder_path)?;

            let result = clause.apply_to_table(table)?;

            let csv = table_to_csv(&result, &result.columns)?;

            clause.write_table(csv, folder_path)?;
        }
        _ => {
            return Err(SqlError::InvalidSyntax);
        }
    }
    Ok(result_csv)
}

fn main() -> Result<(), SqlError> {
    let args: Vec<String> = env::args().collect();

    let result = exec_query(&args[1], &args[2]);

    match result {
        Ok(csv) => {
            for line in csv {
                println!("{}", line);
            }
        }
        Err(e) => println!("{}", e),
    }

    Ok(())
}
