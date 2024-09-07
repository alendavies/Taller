use super::{condition::Condition, recursive_parser::parse_condition};
use crate::{errors::SqlError, utils::is_where};
use std::collections::HashMap;

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

        Ok(Where { condition })
    }

    pub fn execute(&self, register: &HashMap<String, String>) -> Result<bool, SqlError> {
        return self.condition.execute(register);
    }
}
