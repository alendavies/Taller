use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum SqlError {
    InvalidTable,  // relacionado a problemas con el procesamiento de las tablas.
    InvalidColumn, // relacionado a problemas con el procesamiento de columnas.
    InvalidSyntax, // relacionado a problemas con el procesamiento de consultas.
    Error,         // tipo genérico para otros posibles errores detectados.
}

impl Display for SqlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlError::InvalidTable => write!(f, "[InvalidTable]: [Error to open or process table]"),
            SqlError::InvalidColumn => write!(f, "[InvalidColumn]: [Error to process column]"),
            SqlError::InvalidSyntax => write!(f, "[InvalidSyntax]: [Error to process query]"),
            SqlError::Error => write!(f, "[Error]: [Error]"),
        }
    }
}
