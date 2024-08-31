use std::collections::HashMap;

enum Operator {
    Equal,
    Greater,
    Less,
}

pub struct Where {
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
            println!("Error al crear where");
        }

        let mut column = String::new();
        let mut value = String::new();
        let mut operator = Operator::Equal;
        let mut i = 0;

        while i < tokens.len() {
            let char = tokens[i].chars().nth(0).unwrap_or('0');
            if is_operator(char) {
                column = tokens[i - 1].to_string();
                value = tokens[i + 1].to_string();
                operator = match tokens[i] {
                    "=" => Operator::Equal,
                    ">" => Operator::Greater,
                    "<" => Operator::Less,
                    _ => panic!("Operador no soportado"),
                }
            }
            i += 1;
        }

        Self {
            column,
            operator,
            value,
        }
    }

    pub fn execute(&self, register: &HashMap<String, String>) -> bool {
        let default = String::new();
        let x = register.get(&self.column).unwrap_or(&default);
        let y = &self.value;

        let op_result = match self.operator {
            Operator::Less => *x < *y,
            Operator::Greater => *x > *y,
            Operator::Equal => *x == *y,
        };

        op_result
    }
}
