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

    pub fn new_simple_from_tokens(
        tokens: &Vec<&str>,
        pos: usize,
    ) -> Result<(Self, usize), SqlError> {
        if let Some(field) = tokens.get(pos) {
            let pos = pos + 1; // Consume field

            if let Some(operator) = tokens.get(pos) {
                let pos = pos + 1; // Consume operator

                if let Some(value) = tokens.get(pos) {
                    let pos = pos + 1; // Consume value
                    Ok((Condition::new_simple(field, operator, value)?, pos))
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

pub fn parse_condition(tokens: &Vec<&str>, pos: usize) -> Result<(Condition, usize), SqlError> {
    let (mut left, mut pos) = parse_or(tokens, pos)?;

    while let Some(token) = tokens.get(pos) {
        if is_or(token) {
            pos += 1; // Consume "OR"
            let (right, new_pos) = parse_or(tokens, pos)?; // Parse right-hand side
            left = Condition::new_complex(Some(left), LogicalOperator::Or, right);
            pos = new_pos;
        } else {
            break;
        }
    }

    Ok((left, pos))
}

fn parse_or(tokens: &Vec<&str>, pos: usize) -> Result<(Condition, usize), SqlError> {
    let (mut left, mut pos) = parse_and(tokens, pos)?;

    while let Some(token) = tokens.get(pos) {
        if is_and(token) {
            pos += 1; // Consume "AND"
            let (right, new_pos) = parse_and(tokens, pos)?; // Parse right-hand side
            left = Condition::new_complex(Some(left), LogicalOperator::And, right);
            pos = new_pos;
        } else {
            break;
        }
    }

    Ok((left, pos))
}

fn parse_and(tokens: &Vec<&str>, pos: usize) -> Result<(Condition, usize), SqlError> {
    if let Some(token) = tokens.get(pos) {
        if is_not(token) {
            let (expr, new_pos) = parse_condition(tokens, pos + 1)?; // Consume "NOT" and parse next condition
            Ok((
                Condition::new_complex(None, LogicalOperator::Not, expr),
                new_pos,
            ))
        } else {
            parse_base(tokens, pos)
        }
    } else {
        parse_base(tokens, pos) // Handle end of tokens gracefully
    }
}

fn parse_base(tokens: &Vec<&str>, pos: usize) -> Result<(Condition, usize), SqlError> {
    if let Some(token) = tokens.get(pos) {
        if is_left_paren(token) {
            let (expr, new_pos) = parse_condition(tokens, pos + 1)?; // Parse the inner expression
            let next_token = tokens.get(new_pos).ok_or(SqlError::Error)?;
            if is_right_paren(&next_token) {
                Ok((expr, new_pos + 1)) // Consume ")" and return the parsed condition
            } else {
                Err(SqlError::Error)
            }
        } else {
            let (simple_condition, pos) = Condition::new_simple_from_tokens(tokens, pos)?;
            Ok((simple_condition, pos))
        }
    } else {
        Err(SqlError::Error)
    }
}

#[cfg(test)]
mod tests {
    use crate::clauses::recursive_parser::{parse_condition, Condition, LogicalOperator, Operator};

    #[test]
    fn simple_condition() {
        // city = 'Gaiman'
        let tokens = vec!["city", "=", "Gaiman"];
        let pos = 0;
        let (condition, _) = parse_condition(&tokens, pos).unwrap();

        assert_eq!(
            condition,
            Condition::Simple {
                field: String::from("city"),
                operator: Operator::Equal,
                value: String::from("Gaiman"),
            }
        )
    }

    #[test]
    fn simple_or() {
        // city = 'Gaiman' OR age < 30
        let tokens = vec!["city", "=", "Gaiman", "OR", "age", "<", "30"];
        let pos = 0;
        let (condition, _) = parse_condition(&tokens, pos).unwrap();
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
    fn double_or() {
        // city = 'Gaiman' OR age < 30 OR lastname = 'Davies'
        let tokens = vec![
            "city", "=", "Gaiman", "OR", "age", "<", "30", "OR", "lastname", "=", "Davies",
        ];
        let pos = 0;
        let (condition, _) = parse_condition(&tokens, pos).unwrap();
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
    fn simple_and() {
        // city = 'Gaiman' AND age < 30
        let tokens = vec!["city", "=", "Gaiman", "AND", "age", "<", "30"];
        let pos = 0;
        let (condition, _) = parse_condition(&tokens, pos).unwrap();
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
    fn double_and() {
        // city = 'Gaiman' AND age < 30 AND lastname = 'Davies'
        let tokens = vec![
            "city", "=", "Gaiman", "AND", "age", "<", "30", "AND", "lastname", "=", "Davies",
        ];
        let pos = 0;
        let (condition, _) = parse_condition(&tokens, pos).unwrap();
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

    /* #[test]
       fn simple_not() {
           // NOT name = "Luca"
           let mut parser = Parser::new(vec![
               Token::Not,
               Token::Identifier("name".to_string()),
               Token::Equal,
               Token::String("Luca".to_string()),
           ]);
           let expression = expression(&mut parser).unwrap();
           assert_eq!(
               expression,
               Restriction::Not(Box::new(Restriction::Comparison(Comparison::Equal {
                   column: "name".to_string(),
                   value: Value::String("Luca".to_string())
               })))
           )
       }

       #[test]
       fn nested_not_is_fine() {
           // NOT NOT NOT name = "Luca"
           let mut parser = Parser::new(vec![
               Token::Not,
               Token::Not,
               Token::Not,
               Token::Identifier("name".to_string()),
               Token::Equal,
               Token::String("Luca".to_string()),
           ]);
           let expression = expression(&mut parser).unwrap();
           assert_eq!(
               expression,
               Restriction::Not(Box::new(Restriction::Not(Box::new(Restriction::Not(
                   Box::new(Restriction::Comparison(Comparison::Equal {
                       column: "name".to_string(),
                       value: Value::String("Luca".to_string())
                   }))
               )))))
           )
       }

       #[test]
       fn mixed_and_or_not() {
           // name = "Luca" AND padron = 107044 OR age > 23 AND materia = "taller"
           // (name = "Luca" AND padron = 107044)
           let mut parser = Parser::new(vec![
               Token::Identifier("name".to_string()),
               Token::Equal,
               Token::String("Luca".to_string()),
               Token::And,
               Token::Identifier("padron".to_string()),
               Token::Equal,
               Token::Integer(107044),
               Token::Or,
               Token::Identifier("age".to_string()),
               Token::Greater,
               Token::Integer(23),
               Token::And,
               Token::Identifier("materia".to_string()),
               Token::Equal,
               Token::String("taller".to_string()),
           ]);
           let expression = expression(&mut parser).unwrap();
           assert_eq!(
               expression,
               Restriction::Or {
                   left: Box::new(Restriction::And {
                       left: Box::new(Restriction::Comparison(Comparison::Equal {
                           column: "name".to_string(),
                           value: Value::String("Luca".to_string())
                       })),
                       right: Box::new(Restriction::Comparison(Comparison::Equal {
                           column: "padron".to_string(),
                           value: Value::Integer(107044)
                       }))
                   }),
                   right: Box::new(Restriction::And {
                       left: Box::new(Restriction::Comparison(Comparison::Greater {
                           column: "age".to_string(),
                           value: Value::Integer(23)
                       })),
                       right: Box::new(Restriction::Comparison(Comparison::Equal {
                           column: "materia".to_string(),
                           value: Value::String("taller".to_string())
                       })),
                   })
               }
           )
           //         assertion `left == right` failed
           //   left: Or { left: And { left: Comparison(Equal { column: "name", value: String("Luca") }), right: Comparison(Equal { column: "padron", value: Integer(107044) }) }, right: And { left: Comparison(Greater { column: "age", value: Integer(23) }), right: Comparison(Equal { column: "materia", value: String("taller") }) } }
           //  right: Or { left: And { left: Comparison(Equal { column: "name", value: String("Luca") }), right: Comparison(Equal { column: "padron", value: Integer(107044) }) }, right: And { left: Comparison(Equal { column: "materia", value: String("taller") }), right: Comparison(Greater { column: "age", value: Integer(23) }) } }
       }
    */
    /* #[test]
    fn nested_and_or_not() {
        //
    }

    #[test]
    fn parens_respect_precedence() {
        // birth = "SF" AND name = "Baker" OR "Yorgos"
        // equals
        // (birth = "SF" AND name = "Baker") OR "Yorgos"
    }

    #[test]
    fn parens_respect_precedence_2() {
        // birth = "SF" OR name = "Baker" AND name = "Yorgos"
        // equals
        // birth = "SF" OR (name = "Baker" AND name = "Yorgos")
    }

    #[test]
    fn parens_respect_precedence_3() {
        // NOT name = "Luca" AND age > 23
        // equals
        // (NOT name = "Luca") AND age > 23
    }

    #[test]
    fn many_nested_parens_should_be_fine() {
        // NOT ((( NOT ((( NOT name = "Luca" ))))))
    }

    #[test]
    fn simple_parens() {
        // (name = "Luca" OR padron = 107044) AND age > 23
    }

    #[test]
    fn nested_parens() {
        // NOT (name = "Luca" AND (age > 23 OR padron = 107044))
    }

    #[test]
    fn missing_closing_paren() {
        // (NOT name = "Luca" AND age > 23
        // fails
    }

    #[test]
    fn unexpected_token() {
        // NOT AND name = "Luca"
        // fails
    }

    #[test]
    fn missing_side() {
        // name = "Luca" OR
        // fails
    }

    #[test]
    fn yikes() {
        // name = "Luca" AND NOT age > 23 OR (padron = 107044 AND NOT name = "Luca")
    }

    #[test]
    fn simple_comparison_inside_parens() {
        todo!()
    }

    #[test]
    fn primary_unexpected_token() {
        todo!()
    }

    #[test]
    fn missing_parens() {
        todo!()
    }

    #[test]
    fn multiple_nested_parens() {
        todo!()
    } */
}
