use crate::{errors::SqlError, utils::is_set};

#[derive(PartialEq, Debug)]
pub struct Set(pub Vec<(String, String)>);

impl Set {
    pub fn new_from_tokens(tokens: Vec<&str>) -> Result<Self, SqlError> {
        let mut set = Vec::new();
        let mut i = 0;

        if !is_set(tokens[i]) || !tokens.contains(&"=") {
            return Err(SqlError::InvalidSyntax);
        }
        i += 1;

        while i < tokens.len() {
            if tokens[i] == "=" && i + 1 < tokens.len() {
                set.push((tokens[i - 1].to_string(), tokens[i + 1].to_string()));
            }
            i += 1;
        }

        Ok(Self(set))
    }
}
