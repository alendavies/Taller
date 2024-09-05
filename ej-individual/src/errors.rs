#[derive(Debug)]
pub enum SqlError {
    InvalidTable,  // relacionado a problemas con el procesamiento de las tablas.
    InvalidColumn, // relacionado a problemas con el procesamiento de columnas.
    InvalidSyntax, // relacionado a problemas con el procesamiento de consultas.
    Error,         // tipo genérico para otros posibles errores detectados.
}
