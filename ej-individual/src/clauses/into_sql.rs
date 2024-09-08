use crate::{errors::SqlError, utils::is_into};

pub struct Into {
    pub table_name: String,
    pub columns: Vec<String>,
}

impl Into {
    pub fn new_from_tokens(tokens: Vec<&str>) -> Result<Self, SqlError> {
        if tokens.len() < 3 {
            return Err(SqlError::InvalidSyntax);
        }
        let mut i = 0;
        let mut table_name = String::new();
        let mut columns: Vec<String> = Vec::new();

        if is_into(&tokens[i]) {
            i += 1;
            table_name = tokens[i].to_string();
            i += 1;

            let cols: Vec<String> = tokens[i].split(",").map(|c| c.trim().to_string()).collect();

            for col in cols {
                columns.push(col);
            }
        } else {
            return Err(SqlError::InvalidSyntax);
        }

        Ok(Self {
            table_name,
            columns,
        })
    }
}
