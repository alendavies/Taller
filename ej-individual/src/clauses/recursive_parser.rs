use crate::{
    errors::SqlError,
    utils::{is_and, is_left_paren, is_not, is_or, is_right_paren},
};

#[derive(Debug, PartialEq)]
pub enum LogicalOperator {
    And,
    Or,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Equal,
    Greater,
    Lesser,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum Condition {
    Simple {
        field: String,
        operator: Operator,
        value: String,
    },
    Complex {
        left: Option<Box<Condition>>, // Opcional para el caso de 'Not'
        operator: LogicalOperator,
        right: Box<Condition>,
    },
}

impl Condition {
    pub fn new_simple(field: &str, operator: &str, value: &str) -> Result<Self, SqlError> {
        let op: Operator = match operator {
            "=" => Operator::Equal,
            ">" => Operator::Greater,
            "<" => Operator::Lesser,
            _ => Operator::Unknown,
        };

        if op == Operator::Unknown {
            println!("Error en operador");
            return Err(SqlError::InvalidSyntax);
        }

        Ok(Condition::Simple {
            field: field.to_string(),
            operator: op,
            value: value.to_string(),
        })
    }

    pub fn new_complex(
        left: Option<Condition>,
        operator: LogicalOperator,
        right: Condition,
    ) -> Self {
        Condition::Complex {
            left: left.map(Box::new),
            operator,
            right: Box::new(right),
        }
    }

    pub fn new_simple_from_tokens(tokens: &Vec<&str>, pos: &mut usize) -> Result<Self, SqlError> {
        if let Some(field) = tokens.get(*pos) {
            *pos += 1; // Consume field

            if let Some(operator) = tokens.get(*pos) {
                *pos += 1; // Consume operator

                if let Some(value) = tokens.get(*pos) {
                    *pos += 1; // Consume value
                    Ok(Condition::new_simple(field, operator, value)?)
                } else {
                    Err(SqlError::Error)
                }
            } else {
                Err(SqlError::Error)
            }
        } else {
            Err(SqlError::Error)
        }
    }
}

pub fn parse_condition(tokens: &Vec<&str>, pos: &mut usize) -> Result<Condition, SqlError> {
    let mut left = parse_or(tokens, pos)?;

    while let Some(token) = tokens.get(*pos) {
        if is_or(token) {
            *pos += 1; // Consume "OR"
            let right = parse_or(tokens, pos)?; // Parse right-hand side
            left = Condition::new_complex(Some(left), LogicalOperator::Or, right);
        } else {
            break;
        }
    }

    Ok(left)
}

fn parse_or(tokens: &Vec<&str>, pos: &mut usize) -> Result<Condition, SqlError> {
    let mut left = parse_and(tokens, pos)?;

    while let Some(token) = tokens.get(*pos) {
        if is_and(token) {
            *pos += 1; // Consume "AND"
            let right = parse_and(tokens, pos)?; // Parse right-hand side
            left = Condition::new_complex(Some(left), LogicalOperator::And, right);
        } else {
            break;
        }
    }

    Ok(left)
}

fn parse_and(tokens: &Vec<&str>, pos: &mut usize) -> Result<Condition, SqlError> {
    if let Some(token) = tokens.get(*pos) {
        if is_not(token) {
            *pos += 1;
            let expr = parse_condition(tokens, pos)?; // Consume "NOT" and parse next condition
            Ok(Condition::new_complex(None, LogicalOperator::Not, expr))
        } else {
            parse_base(tokens, pos)
        }
    } else {
        parse_base(tokens, pos) // Handle end of tokens gracefully
    }
}

fn parse_base(tokens: &Vec<&str>, pos: &mut usize) -> Result<Condition, SqlError> {
    if let Some(token) = tokens.get(*pos) {
        if is_left_paren(token) {
            *pos += 1;
            let expr = parse_condition(tokens, pos)?; // Parse the inner expression
            let next_token = tokens.get(*pos).ok_or(SqlError::Error)?;
            if is_right_paren(&next_token) {
                *pos += 1;
                Ok(expr) // Consume ")" and return the parsed condition
            } else {
                Err(SqlError::Error)
            }
        } else {
            let simple_condition = Condition::new_simple_from_tokens(tokens, pos)?;
            Ok(simple_condition)
        }
    } else {
        Err(SqlError::Error)
    }
}

#[cfg(test)]
mod tests {
    use crate::clauses::recursive_parser::{parse_condition, Condition, LogicalOperator, Operator};

    #[test]
    fn simple_conditions() {
        let tokens1 = vec!["city", "=", "Gaiman"];
        let tokens2 = vec!["age", "<", "30"];
        let tokens3 = vec!["age", ">", "18"];
        let mut pos = 0;
        let condition1 = parse_condition(&tokens1, &mut pos).unwrap();
        pos = 0;
        let condition2 = parse_condition(&tokens2, &mut pos).unwrap();
        pos = 0;
        let condition3 = parse_condition(&tokens3, &mut pos).unwrap();

        assert_eq!(
            condition1,
            Condition::Simple {
                field: String::from("city"),
                operator: Operator::Equal,
                value: String::from("Gaiman"),
            }
        );
        assert_eq!(
            condition2,
            Condition::Simple {
                field: String::from("age"),
                operator: Operator::Lesser,
                value: String::from("30"),
            }
        );
        assert_eq!(
            condition3,
            Condition::Simple {
                field: String::from("age"),
                operator: Operator::Greater,
                value: String::from("18"),
            }
        );
    }

    #[test]
    fn not() {
        let tokens = vec!["NOT", "city", "=", "Gaiman"];
        let mut pos = 0;
        let condition = parse_condition(&tokens, &mut pos).unwrap();
        assert_eq!(
            condition,
            Condition::Complex {
                left: None,
                operator: LogicalOperator::Not,
                right: Box::new(Condition::Simple {
                    field: String::from("city"),
                    operator: Operator::Equal,
                    value: String::from("Gaiman")
                })
            }
        )
    }

    #[test]
    fn one_or() {
        let tokens = vec!["city", "=", "Gaiman", "OR", "age", "<", "30"];
        let mut pos = 0;
        let condition = parse_condition(&tokens, &mut pos).unwrap();
        assert_eq!(
            condition,
            Condition::Complex {
                left: Some(Box::new(Condition::Simple {
                    field: String::from("city"),
                    operator: Operator::Equal,
                    value: String::from("Gaiman")
                })),
                operator: LogicalOperator::Or,
                right: Box::new(Condition::Simple {
                    field: String::from("age"),
                    operator: Operator::Lesser,
                    value: String::from("30")
                })
            }
        )
    }

    #[test]
    fn two_or() {
        let tokens = vec![
            "city", "=", "Gaiman", "OR", "age", "<", "30", "OR", "lastname", "=", "Davies",
        ];
        let mut pos = 0;
        let condition = parse_condition(&tokens, &mut pos).unwrap();
        assert_eq!(
            condition,
            Condition::Complex {
                left: Some(Box::new(Condition::Complex {
                    left: Some(Box::new(Condition::Simple {
                        field: String::from("city"),
                        operator: Operator::Equal,
                        value: String::from("Gaiman")
                    })),
                    operator: LogicalOperator::Or,
                    right: Box::new(Condition::Simple {
                        field: String::from("age"),
                        operator: Operator::Lesser,
                        value: String::from("30")
                    })
                })),
                operator: LogicalOperator::Or,
                right: Box::new(Condition::Simple {
                    field: String::from("lastname"),
                    operator: Operator::Equal,
                    value: String::from("Davies")
                })
            }
        )
    }

    #[test]
    fn one_and() {
        let tokens = vec!["city", "=", "Gaiman", "AND", "age", "<", "30"];
        let mut pos = 0;
        let condition = parse_condition(&tokens, &mut pos).unwrap();
        assert_eq!(
            condition,
            Condition::Complex {
                left: Some(Box::new(Condition::Simple {
                    field: String::from("city"),
                    operator: Operator::Equal,
                    value: String::from("Gaiman")
                })),
                operator: LogicalOperator::And,
                right: Box::new(Condition::Simple {
                    field: String::from("age"),
                    operator: Operator::Lesser,
                    value: String::from("30")
                })
            }
        )
    }

    #[test]
    fn two_and() {
        let tokens = vec![
            "city", "=", "Gaiman", "AND", "age", "<", "30", "AND", "lastname", "=", "Davies",
        ];
        let mut pos = 0;
        let condition = parse_condition(&tokens, &mut pos).unwrap();
        assert_eq!(
            condition,
            Condition::Complex {
                left: Some(Box::new(Condition::Complex {
                    left: Some(Box::new(Condition::Simple {
                        field: String::from("city"),
                        operator: Operator::Equal,
                        value: String::from("Gaiman")
                    })),
                    operator: LogicalOperator::And,
                    right: Box::new(Condition::Simple {
                        field: String::from("age"),
                        operator: Operator::Lesser,
                        value: String::from("30")
                    })
                })),
                operator: LogicalOperator::And,
                right: Box::new(Condition::Simple {
                    field: String::from("lastname"),
                    operator: Operator::Equal,
                    value: String::from("Davies")
                })
            }
        )
    }

    #[test]
    fn and_or() {
        let tokens = vec![
            "city", "=", "Gaiman", "AND", "age", ">", "18", "OR", "lastname", "=", "Davies",
        ];
        let mut pos = 0;
        let condition = parse_condition(&tokens, &mut pos).unwrap();
        assert_eq!(
            condition,
            Condition::Complex {
                left: Some(Box::new(Condition::Complex {
                    left: Some(Box::new(Condition::Simple {
                        field: String::from("city"),
                        operator: Operator::Equal,
                        value: String::from("Gaiman")
                    })),
                    operator: LogicalOperator::And,
                    right: Box::new(Condition::Simple {
                        field: String::from("age"),
                        operator: Operator::Greater,
                        value: String::from("18")
                    })
                })),
                operator: LogicalOperator::Or,
                right: Box::new(Condition::Simple {
                    field: String::from("lastname"),
                    operator: Operator::Equal,
                    value: String::from("Davies")
                })
            }
        )
    }
}
