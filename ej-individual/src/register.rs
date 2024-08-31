use std::collections::HashMap;

use crate::CustomError;

#[derive(Clone, Debug)]
pub struct Register(pub HashMap<String, String>);

impl Register {
    pub fn to_csv(&self, columns: &Vec<String>) -> Result<String, CustomError> {
        let mut values = Vec::new();

        for col in columns {
            let value = self.0.get(col).ok_or(CustomError::CsvError)?;
            values.push(value.to_string());
        }

        let csv = values.join(",");

        Ok(csv)
    }
}
