use core::panic;
use std::{
    collections::HashMap,
    default,
    fs::File,
    io::{self, BufRead, BufReader, Error},
    result,
};

enum CustomError {
    ReaderError,
    Main_Error,
}

struct Table {
    columns: Vec<String>,
    registers: Vec<HashMap<String, String>>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            registers: Vec::new(),
        }
    }
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

fn equal_to_str(a: &str, b: &str) -> bool {
    a == b
}

fn equal_to_num(a: i32, b: i32) -> bool {
    a == b
}

fn greater_than(a: i32, b: i32) -> bool {
    a > b
}

fn less_than(a: i32, b: i32) -> bool {
    a < b
}

impl Where {
    pub fn new_from_tokens(tokens: Vec<&str>) -> Self {
        if tokens.len() < 1 {
            println!("Error al crear where");
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

    fn execute(&self, register: &HashMap<String, String>) -> bool {
        let default = String::new();
        let x = register.get(&self.column).unwrap_or(&default);
        let y = &self.value;

        let op_result = match self.operator {
            Operator::Less => x < y,
            Operator::Greater => x > y,
            Operator::Equal => x == y,
        };

        op_result
    }
}

struct OrderBy {
    column: String,
    order: String,
}

impl OrderBy {
    pub fn new_from_tokens(tokens: Vec<&str>) -> Self {
        let mut column = String::new();
        let mut order = String::new();

        if tokens.len() < 1 {
            return Self { column, order };
        }

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

    fn execute(
        &self,
        registers: &mut Vec<HashMap<String, String>>,
    ) -> Vec<HashMap<String, String>> {
        registers.sort_by(|a, b| {
            let default = String::new();
            let val_a = a.get(&self.column).unwrap_or(&default);
            let val_b = b.get(&self.column).unwrap_or(&default);
            if self.order == "DESC" {
                val_b.cmp(val_a)
            } else {
                val_a.cmp(val_b)
            }
        });
        registers.to_vec()
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

    fn execute(&self, line: String, columns: &Vec<String>) -> HashMap<String, String> {
        let atributes: Vec<String> = line.split(',').map(|s| s.to_string()).collect();

        let mut register = HashMap::new();

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

        let mut result = HashMap::new();
        let op_result = self.where_clause.execute(&register);

        if op_result == true {
            for col in col_selected {
                result.insert(
                    col.to_string(),
                    register.get(&col).unwrap_or(&String::new()).to_string(),
                );
            }
        }

        result
    }
}

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
                current.push(char);
                index += 1;
                char = string.chars().nth(index).unwrap_or('0');

                while char != '\'' && index < length {
                    current.push(char);
                    index += 1;
                    char = string.chars().nth(index).unwrap_or('0');
                }
                current.push(char);

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

fn exec_query(file: File, query: &str) -> Result<Table, CustomError> {
    let reader = BufReader::new(file);
    let tokens = parse(query);
    let clause;
    let mut result = Table::new();

    match tokens[0].as_str() {
        "SELECT" => {
            clause = Select::new_from_tokens(tokens);
            for (idx, line) in reader.lines().enumerate() {
                let line = line.map_err(|_| CustomError::ReaderError)?;
                if idx == 0 {
                    result.columns = line.split(',').map(|s| s.to_string()).collect();
                    continue;
                }
                let register = clause.execute(line, &result.columns);
                if !register.is_empty() {
                    result.registers.push(register);
                }
            }
            println!("{:?}", result.registers);

            if clause.orderby_clause.column != "" {
                let registers_ordered = clause.orderby_clause.execute(&mut result.registers);
                result.registers = registers_ordered;
            }
        }
        _ => todo!(),
    }

    Ok(result)
}

fn main() -> io::Result<()> {
    let example = vec![
        "tabla.csv",
        "SELECT id, nombre, email FROM clientes WHERE apellido = 'López' ORDER BY email DESC;",
    ];

    let table = example[0];
    let query = example[1];

    let file = File::open(table)?;

    match exec_query(file, query) {
        Ok(_) => Ok(println!("Todo ok")),
        Err(_) => todo!("Algo falló"),
    }
}
