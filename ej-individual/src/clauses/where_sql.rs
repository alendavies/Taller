use std::{collections::HashMap, panic::Location};

use crate::{errors::SqlError, utils::is_where};

use super::recursive_parser::{parse_condition, Condition, LogicalOperator, Operator};

pub struct Where {
    pub condition: Condition,
}

impl Where {
    pub fn new_from_tokens(tokens: Vec<&str>) -> Result<Self, SqlError> {
        if !is_where(tokens[0]) {
            return Err(SqlError::Error);
        }
        let mut pos = 1;
        let condition = parse_condition(&tokens, &mut pos)?;
        println!("{:?}", condition);

        Ok(Where { condition })
    }

    pub fn execute(&self, register: &HashMap<String, String>) -> Result<bool, SqlError> {
        let op_result: Result<bool, SqlError> = match &self.condition {
            Condition::Simple {
                field,
                operator,
                value,
            } => {
                let y = value;
                if let Some(x) = register.get(field) {
                    return match operator {
                        Operator::Lesser => Ok(x < y),
                        Operator::Greater => Ok(x > y),
                        Operator::Equal => Ok(x == y),
                        Operator::Unknown => Err(SqlError::Error),
                    };
                } else {
                    return Err(SqlError::Error);
                }
            }
            Condition::Complex {
                left,
                operator,
                right,
            } => match operator {
                LogicalOperator::Not => todo!(),
                LogicalOperator::Or => todo!(),
                LogicalOperator::And => todo!(),
            },
        };

        op_result
    }
}
