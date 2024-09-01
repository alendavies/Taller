#[derive(Debug)]
pub enum SqlError {
    InvalidTable,       // relacionado a problemas con el procesamiento de las tablas.
    InvalidColumn,      // relacionado a problemas con el procesamiento de columnas.
    InvalidSyntax,      // relacionado a problemas con el procesamiento de consultas.
    Error(CustomError), // tipo genérico para otros posibles errores detectados.
}

#[derive(Debug)]
pub enum CustomError {
    ReaderError,
    WriteError,
    FileError,
    HashError,
}
