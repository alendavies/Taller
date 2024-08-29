use core::panic;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Error},
};

enum Operator {
    Equal,
    Greater,
    Less,
}

struct Where {
    column: String,
    operator: Operator,
    value: String,
}

impl Where {
    pub fn new(tokens: Vec<&str>) -> Self {
        if tokens.len() < 1 {
            println!("Error en clausula where");
        }
        let operator = match tokens[1] {
            ">" => Operator::Greater,
            "<" => Operator::Less,
            "=" => Operator::Equal,
            _ => panic!("Operador no soportado"),
        };

        Self {
            column: tokens[0].to_string(),
            operator,
            value: tokens[2].to_string(),
        }
    }
}

struct OrderBy {
    column: String,
    order: String,
}

impl OrderBy {
    pub fn new(tokens: Vec<&str>) -> Self {
        if tokens.len() < 1 {
            println!("Error en clausula orderby");
        }
        let column = tokens[0].to_string();
        let mut order = String::new();

        if tokens.len() == 2 {
            order = tokens[1].to_string();
        }

        Self { column, order }
    }
}

struct Select {
    columns: Vec<String>,
    where_clause: Where,
    orderby_clause: OrderBy,
}

impl Select {
    pub fn new(tokens: Vec<String>) {
        if !tokens.contains(&String::from("WHERE")) || !tokens.contains(&String::from("SELECT")) {
            println!("Clausula SELECT inválida");
        }

        let mut columns: Vec<&str> = Vec::new();
        let mut where_tokens: Vec<&str> = Vec::new();
        let mut orderby_tokens: Vec<&str> = Vec::new();

        let mut i = 0;

        if tokens[i] == "SELECT" {
            i += 1;
            while tokens[i] != "WHERE" {
                columns.push(tokens[i].as_str());
                i += 1;
            }
        }
        if tokens[i] == "WHERE" {
            i += 1;
            while tokens[i] != "ORDER" && i < tokens.len() {
                where_tokens.push(tokens[i].as_str());
                i += 1;
            }
        }
        if i < tokens.len() && tokens[i] == "ORDER" && tokens[i + 1] == "BY" {
            i += 2;
            while i < tokens.len() {
                orderby_tokens.push(tokens[i].as_str());
                i += 1;
            }
        }
        Where::new(where_tokens);
        OrderBy::new(orderby_tokens);
    }
}

struct Table {
    columns: Vec<String>,
    registers: Vec<HashMap<String, String>>,
}

enum CustomError {
    ReaderError,
    Main_Error,
}

fn select(query: Select, reader: BufReader<File>) -> Result<Vec<String>, CustomError> {
    let result = Vec::new();
    let where_value = query.where_clause.value.parse::<i32>().unwrap_or(0);

    let mut iter = reader.lines();

    let mut columns;

    if let Some(first_row) = iter.next() {
        columns = first_row
            .map_err(|_| CustomError::ReaderError)?
            .split(",")
            .filter(|col| query.columns.contains(&col.to_string()))
            .map(|c| c.to_string())
            .collect();
    } else {
        return Err(CustomError::ReaderError);
    }

    let table = Table {
        columns,
        registers: Vec::new(),
    };

    for line in iter {
        let line = line.map_err(|_| CustomError::ReaderError)?;
        let tokens: Vec<String> = line.split(",").map(|c| c.to_string()).collect();
    }

    // serialize output before return

    Ok(result)
}

fn parse(string: &str) -> Vec<String> {
    let length = string.len();
    let mut index = 0;
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut char = string.chars().nth(index).unwrap_or('0');

    while index < length {
        if char.is_alphanumeric() || char == '_' && index < length {
            while char.is_alphabetic() || char == '_' {
                current.push(char);
                index += 1;
                char = string.chars().nth(index).unwrap_or('0');
            }
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
                current.push(char);
                index += 1;
                char = string.chars().nth(index).unwrap_or('0');

                while char != '\'' {
                    current.push(char);
                    index += 1;
                    char = string.chars().nth(index).unwrap_or('0');
                }
                current.push(char);

                tokens.push(current);
                current = String::new();
                index += 1;
            } else {
                while !char.is_alphanumeric() && !char.is_whitespace() && index < length {
                    current.push(char);
                    index += 1;
                    char = string.chars().nth(index).unwrap_or('0');
                }
                tokens.push(current);
                if char.is_whitespace() {
                    index += 1;
                    char = string.chars().nth(index).unwrap_or('0');
                }
                current = String::new();
            }
        }
    }

    tokens
}

fn exec_query(file: File, query: &str) -> Result<Vec<String>, CustomError> {
    let tokens: Vec<&str> = query.split_whitespace().collect();
    let reader = BufReader::new(file);
    //let mut clause;
    let result = Vec::new();
    let tokens = parse(query);

    match tokens[0].as_str() {
        "SELECT" => {
            Select::new(tokens);

            // result = select(clause, reader).map_err(|_| CustomError::ReaderError)?;
        }
        _ => todo!(),
    }

    Ok(result)
}

fn main() -> io::Result<()> {
    let example = vec![
        "tabla.csv",
        "SELECT id, producto WHERE cantidad > 1 ORDER BY email",
    ];

    let table = example[0];
    let query = example[1];

    let file = File::open(table)?;

    match exec_query(file, query) {
        Ok(_) => Ok(println!("Todo ok")),
        Err(_) => todo!("Algo falló"),
    }
}
