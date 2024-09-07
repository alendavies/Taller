use super::{condition::Condition, recursive_parser::parse_condition};
use crate::{errors::SqlError, register::Register};

pub struct Where {
    pub condition: Condition,
}

impl Where {
    pub fn new_from_tokens(tokens: Vec<&str>) -> Result<Self, SqlError> {
        if tokens.len() < 4 {
            return Err(SqlError::InvalidSyntax);
        }
        let mut pos = 1;
        let condition = parse_condition(&tokens, &mut pos)?;

        Ok(Self { condition })
    }

    pub fn execute(&self, register: &Register) -> Result<bool, SqlError> {
        return self.condition.execute(&register.0);
    }
}
