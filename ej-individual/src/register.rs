use crate::errors::SqlError;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Register(pub HashMap<String, String>);

impl Register {
    pub fn to_csv(&self, columns: &Vec<String>) -> Result<String, SqlError> {
        let mut values = Vec::new();

        for col in columns {
            let value = self.0.get(col).ok_or(SqlError::Error)?;
            values.push(value.to_string());
        }

        let csv = values.join(",");

        Ok(csv)
    }
}
