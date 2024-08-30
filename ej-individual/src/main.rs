use core::panic;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Error},
};

enum CustomError {
    ReaderError,
    Main_Error,
}

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

fn is_operator(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>')
}

impl Where {
    pub fn new_from_tokens(tokens: Vec<&str>) -> Self {
        if tokens.len() < 1 {
            println!("Error en clausula where");
        }
        let mut column = String::new();
        let mut value = String::new();
        let mut operator = Operator::Equal;

        for token in tokens {
            if token.chars().all(|c| c.is_alphabetic()) {
                column = token.to_string();
            } else if token.chars().all(|c| c.is_numeric())
                || (token.starts_with('\'') && token.ends_with('\''))
            {
                value = token.to_string();
            } else if token.chars().all(|c| is_operator(c)) {
                operator = match token {
                    "=" => Operator::Equal,
                    ">" => Operator::Greater,
                    "<" => Operator::Less,
                    _ => panic!("Operador no soportado"),
                };
            } else {
                println!("Error en clausula where");
            }
        }

        Self {
            column,
            operator,
            value,
        }
    }
}

struct OrderBy {
    column: String,
    order: String,
}

impl OrderBy {
    pub fn new_from_tokens(tokens: Vec<&str>) -> Self {
        if tokens.len() < 1 {
            println!("Error en clausula orderby");
        }

        let mut column = String::new();
        let mut order = String::new();

        for token in tokens {
            if token.chars().all(|c| c.is_alphabetic()) {
                if token.chars().all(|c| c.is_lowercase()) {
                    column = token.to_string();
                } else if token.chars().all(|c| c.is_uppercase()) {
                    order = token.to_string();
                }
            }
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
    pub fn new_from_tokens(tokens: Vec<String>) -> Self {
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

        let where_clause = Where::new_from_tokens(where_tokens);
        let orderby_clause = OrderBy::new_from_tokens(orderby_tokens);

        Self {
            columns: columns.iter().map(|c| c.to_string()).collect(),
            where_clause,
            orderby_clause,
        }
    }
}

struct Table {
    columns: Vec<String>,
    registers: Vec<HashMap<String, String>>,
}

fn select(query: Select, line: String) -> Result<Vec<String>, CustomError> {
    let result = Vec::new();

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
    let reader = BufReader::new(file);
    let tokens = parse(query);
    let clause;
    let result;

    match tokens[0].as_str() {
        "SELECT" => {
            clause = Select::new_from_tokens(tokens);
            for line in reader.lines() {
                let line = line?;
                result = select(clause, line);
            }
        }
        _ => todo!(),
    }

    Ok(result)
}

fn main() -> io::Result<()> {
    let example = vec![
        "tabla.csv",
        "SELECT id, producto WHERE cantidad > 1 ORDER BY email DESC;",
    ];

    let table = example[0];
    let query = example[1];

    let file = File::open(table)?;

    match exec_query(file, query) {
        Ok(_) => Ok(println!("Todo ok")),
        Err(_) => todo!("Algo falló"),
    }
}
