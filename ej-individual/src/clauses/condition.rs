use crate::errors::SqlError;
use std::collections::HashMap;

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
        let op: Operator;

        match operator {
            "=" => op = Operator::Equal,
            ">" => op = Operator::Greater,
            "<" => op = Operator::Lesser,
            _ => return Err(SqlError::Error),
        };

        Ok(Condition::Simple {
            field: field.to_string(),
            operator: op,
            value: value.to_string(),
        })
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

    pub fn execute(&self, register: &HashMap<String, String>) -> Result<bool, SqlError> {
        let op_result: Result<bool, SqlError> = match &self {
            Condition::Simple {
                field,
                operator,
                value,
            } => {
                let y = value;
                if let Some(x) = register.get(field) {
                    match operator {
                        Operator::Lesser => Ok(x < y),
                        Operator::Greater => Ok(x > y),
                        Operator::Equal => Ok(x == y),
                    }
                } else {
                    Err(SqlError::Error)
                }
            }
            Condition::Complex {
                left,
                operator,
                right,
            } => match operator {
                LogicalOperator::Not => {
                    let result = right.execute(register)?;
                    Ok(!result)
                }
                LogicalOperator::Or => {
                    if let Some(left) = left {
                        let left_result = left.execute(register)?;
                        let right_result = right.execute(register)?;
                        Ok(left_result || right_result)
                    } else {
                        Err(SqlError::Error)
                    }
                }
                LogicalOperator::And => {
                    if let Some(left) = left {
                        let left_result = left.execute(register)?;
                        let right_result = right.execute(register)?;
                        Ok(left_result && right_result)
                    } else {
                        Err(SqlError::Error)
                    }
                }
            },
        };
        op_result
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Condition;
    use crate::clauses::condition::{LogicalOperator, Operator};

    #[test]
    fn create_simple() {
        let condition = Condition::new_simple("age", ">", "18").unwrap();
        assert_eq!(
            condition,
            Condition::Simple {
                field: "age".to_string(),
                operator: Operator::Greater,
                value: "18".to_string()
            }
        )
    }

    #[test]
    fn create_simple_from_tokens() {
        let tokens = vec!["age", ">", "18"];
        let mut pos = 0;
        let condition = Condition::new_simple_from_tokens(&tokens, &mut pos).unwrap();

        assert_eq!(
            condition,
            Condition::Simple {
                field: "age".to_string(),
                operator: Operator::Greater,
                value: "18".to_string()
            }
        )
    }

    #[test]
    fn create_complex_with_left() {
        let left = Condition::Simple {
            field: "age".to_string(),
            operator: Operator::Greater,
            value: "18".to_string(),
        };

        let right = Condition::Simple {
            field: "city".to_string(),
            operator: Operator::Equal,
            value: "Gaiman".to_string(),
        };

        let complex = Condition::new_complex(Some(left), LogicalOperator::And, right);

        assert_eq!(
            complex,
            Condition::Complex {
                left: Some(Box::new(Condition::Simple {
                    field: "age".to_string(),
                    operator: Operator::Greater,
                    value: "18".to_string(),
                })),
                operator: LogicalOperator::And,
                right: Box::new(Condition::Simple {
                    field: "city".to_string(),
                    operator: Operator::Equal,
                    value: "Gaiman".to_string(),
                })
            }
        )
    }

    #[test]
    fn create_complex_without_left() {
        let right = Condition::Simple {
            field: "name".to_string(),
            operator: Operator::Equal,
            value: "Alen".to_string(),
        };

        let complex = Condition::new_complex(None, LogicalOperator::Not, right);

        assert_eq!(
            complex,
            Condition::Complex {
                left: None,
                operator: LogicalOperator::Not,
                right: Box::new(Condition::Simple {
                    field: "name".to_string(),
                    operator: Operator::Equal,
                    value: "Alen".to_string(),
                })
            }
        )
    }

    #[test]
    fn execute_simple() {
        let mut register = HashMap::new();
        register.insert("name".to_string(), "Alen".to_string());
        register.insert("lastname".to_string(), "Davies".to_string());
        register.insert("age".to_string(), "24".to_string());

        let condition_true = Condition::Simple {
            field: "age".to_string(),
            operator: Operator::Greater,
            value: "18".to_string(),
        };

        let condition_false = Condition::Simple {
            field: "age".to_string(),
            operator: Operator::Greater,
            value: "40".to_string(),
        };

        let result_true = condition_true.execute(&register).unwrap();
        let result_false = condition_false.execute(&register).unwrap();

        assert_eq!(result_true, true);

        assert_eq!(result_false, false);
    }

    #[test]
    fn execute_and() {
        let mut register = HashMap::new();
        register.insert("name".to_string(), "Alen".to_string());
        register.insert("lastname".to_string(), "Davies".to_string());
        register.insert("age".to_string(), "24".to_string());

        let left = Condition::Simple {
            field: "age".to_string(),
            operator: Operator::Greater,
            value: "18".to_string(),
        };
        let right = Condition::Simple {
            field: "name".to_string(),
            operator: Operator::Equal,
            value: "Alen".to_string(),
        };

        let condition = Condition::Complex {
            left: Some(Box::new(left)),
            operator: LogicalOperator::And,
            right: Box::new(right),
        };

        let result = condition.execute(&register).unwrap();

        assert_eq!(result, true)
    }

    #[test]
    fn execute_or() {
        let mut register = HashMap::new();
        register.insert("name".to_string(), "Alen".to_string());
        register.insert("lastname".to_string(), "Davies".to_string());
        register.insert("age".to_string(), "24".to_string());

        let left = Condition::Simple {
            field: "age".to_string(),
            operator: Operator::Greater,
            value: "40".to_string(),
        };
        let right = Condition::Simple {
            field: "name".to_string(),
            operator: Operator::Equal,
            value: "Emily".to_string(),
        };

        let condition = Condition::Complex {
            left: Some(Box::new(left)),
            operator: LogicalOperator::Or,
            right: Box::new(right),
        };

        let result = condition.execute(&register).unwrap();

        assert_eq!(result, false)
    }

    #[test]
    fn execute_not() {
        let mut register = HashMap::new();
        register.insert("name".to_string(), "Alen".to_string());
        register.insert("lastname".to_string(), "Davies".to_string());
        register.insert("age".to_string(), "24".to_string());

        let right = Condition::Simple {
            field: "name".to_string(),
            operator: Operator::Equal,
            value: "Emily".to_string(),
        };

        let condition = Condition::Complex {
            left: None,
            operator: LogicalOperator::Not,
            right: Box::new(right),
        };

        let result = condition.execute(&register).unwrap();

        assert_eq!(result, true)
    }

    #[test]
    fn execute_and_or() {
        let mut register = HashMap::new();
        register.insert("name".to_string(), "Alen".to_string());
        register.insert("lastname".to_string(), "Davies".to_string());
        register.insert("age".to_string(), "24".to_string());
        register.insert("city".to_string(), "Gaiman".to_string());

        let left = Condition::Simple {
            field: "age".to_string(),
            operator: Operator::Greater,
            value: "40".to_string(),
        };
        let right1 = Condition::Simple {
            field: "name".to_string(),
            operator: Operator::Equal,
            value: "Alen".to_string(),
        };

        let or = Condition::Complex {
            left: Some(Box::new(left)),
            operator: LogicalOperator::Or,
            right: Box::new(right1),
        };

        let right2 = Condition::Simple {
            field: "city".to_string(),
            operator: Operator::Equal,
            value: "Trelew".to_string(),
        };

        let and = Condition::Complex {
            left: Some(Box::new(or)),
            operator: LogicalOperator::And,
            right: Box::new(right2),
        };

        let result = and.execute(&register).unwrap();

        assert_eq!(result, false)
    }

    #[test]
    fn execute_not_and() {
        let mut register = HashMap::new();
        register.insert("name".to_string(), "Alen".to_string());
        register.insert("lastname".to_string(), "Davies".to_string());
        register.insert("age".to_string(), "24".to_string());
        register.insert("city".to_string(), "Gaiman".to_string());

        let right1 = Condition::Simple {
            field: "age".to_string(),
            operator: Operator::Greater,
            value: "40".to_string(),
        };

        let not = Condition::Complex {
            left: None,
            operator: LogicalOperator::Not,
            right: Box::new(right1),
        };

        let right2 = Condition::Simple {
            field: "city".to_string(),
            operator: Operator::Equal,
            value: "Gaiman".to_string(),
        };

        let and = Condition::Complex {
            left: Some(Box::new(not)),
            operator: LogicalOperator::And,
            right: Box::new(right2),
        };

        let result = and.execute(&register).unwrap();

        assert_eq!(result, true)
    }
}
