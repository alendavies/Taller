use super::{orderby_sql::OrderBy, where_sql::Where};
use crate::{
    errors::SqlError,
    register::Register,
    table::Table,
    utils::{find_file_in_folder, is_from, is_orderby, is_select, is_where},
};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Select {
    pub table_name: String,
    pub columns: Vec<String>,
    pub where_clause: Option<Where>,
    pub orderby_clause: Option<OrderBy>,
}

fn parse_columns<'a>(tokens: &'a Vec<String>, i: &mut usize) -> Result<Vec<&'a String>, SqlError> {
    let mut columns = Vec::new();
    if is_select(&tokens[*i]) {
        if *i < tokens.len() {
            *i += 1;
            while !is_from(&tokens[*i]) && *i < tokens.len() {
                columns.push(&tokens[*i]);
                *i += 1;
            }
        }
    } else {
        return Err(SqlError::InvalidSyntax);
    }
    Ok(columns)
}

fn parse_table_name(tokens: &Vec<String>, i: &mut usize) -> Result<String, SqlError> {
    if *i < tokens.len() && is_from(&tokens[*i]) {
        *i += 1;
        let table_name = tokens[*i].to_string();
        *i += 1;
        Ok(table_name)
    } else {
        return Err(SqlError::InvalidSyntax);
    }
}

fn parse_where_and_orderby<'a>(
    tokens: &'a Vec<String>,
    i: &mut usize,
) -> Result<(Vec<&'a str>, Vec<&'a str>), SqlError> {
    let mut where_tokens = Vec::new();
    let mut orderby_tokens = Vec::new();

    if *i < tokens.len() {
        if is_where(&tokens[*i]) {
            while !is_orderby(&tokens[*i], &tokens[*i + 1]) && *i < tokens.len() {
                where_tokens.push(tokens[*i].as_str());
                *i += 1;
            }
        }
        if is_orderby(&tokens[*i], &tokens[*i + 1]) && *i < tokens.len() {
            while *i < tokens.len() {
                orderby_tokens.push(tokens[*i].as_str());
                *i += 1;
            }
        }
    }
    Ok((where_tokens, orderby_tokens))
}

fn convert_line_to_register(line: String, columns: &Vec<String>) -> Register {
    let attributes: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
    let mut original = Register(HashMap::new());
    for (idx, col) in columns.iter().enumerate() {
        original
            .0
            .insert(col.to_string(), attributes[idx].to_string());
    }

    original
}

impl Select {
    pub fn new_from_tokens(tokens: Vec<String>) -> Result<Self, SqlError> {
        if tokens.len() < 4 {
            return Err(SqlError::InvalidSyntax);
        }

        let mut i = 0;

        let columns = parse_columns(&tokens, &mut i)?;
        let table_name = parse_table_name(&tokens, &mut i)?;

        if columns.is_empty() || table_name.is_empty() {
            return Err(SqlError::InvalidSyntax);
        }

        let (where_tokens, orderby_tokens) = parse_where_and_orderby(&tokens, &mut i)?;

        let where_clause = if !where_tokens.is_empty() {
            Some(Where::new_from_tokens(where_tokens)?)
        } else {
            None
        };

        let orderby_clause = if !orderby_tokens.is_empty() {
            Some(OrderBy::new_from_tokens(orderby_tokens))
        } else {
            None
        };

        Ok(Self {
            table_name,
            columns: columns.iter().map(|c| c.to_string()).collect(),
            where_clause: where_clause,
            orderby_clause: orderby_clause,
        })
    }

    pub fn apply_to_table(&self, table: BufReader<File>) -> Result<Table, SqlError> {
        let mut result = Table::new();

        for (idx, line) in table.lines().enumerate() {
            let line = line.map_err(|_| SqlError::Error)?;
            if idx == 0 {
                result.columns = line.split(',').map(|s| s.to_string()).collect();
                continue;
            }
            let register = self.execute(line, &result.columns)?;

            if !register.0.is_empty() {
                result.registers.push(register);
            }
        }

        if let Some(orderby) = &self.orderby_clause {
            let registers_ordered = orderby.execute(&mut result.registers);
            result.registers = registers_ordered.to_vec();
        }

        Ok(result)
    }

    pub fn execute(&self, line: String, columns: &Vec<String>) -> Result<Register, SqlError> {
        if !self.columns.iter().all(|col| columns.contains(col)) {
            return Err(SqlError::InvalidColumn);
        }

        let mut cols_selected = Vec::new();
        if self.columns[0] == "*" {
            for col in columns {
                cols_selected.push(col.to_string());
            }
        } else {
            for col in &self.columns {
                cols_selected.push(col.to_string());
            }
        }

        let original = convert_line_to_register(line, columns);
        let mut result = Register(HashMap::new());

        if let Some(where_clause) = &self.where_clause {
            let op_result = where_clause.execute(&original)?;
            if op_result == true {
                for col in cols_selected {
                    result.0.insert(
                        col.to_string(),
                        original.0.get(&col).unwrap_or(&String::new()).to_string(),
                    );
                }
            }
        } else {
            for col in cols_selected {
                result.0.insert(
                    col.to_string(),
                    original.0.get(&col).unwrap_or(&String::new()).to_string(),
                );
            }
        }
        Ok(result)
    }

    pub fn open_table(&self, folder_path: &str) -> Result<BufReader<File>, SqlError> {
        let table_name = self.table_name.to_string() + ".csv";
        if !find_file_in_folder(folder_path, &table_name) {
            return Err(SqlError::InvalidTable);
        }
        let table_path = folder_path.to_string() + "/" + &table_name;
        let file = File::open(&table_path).map_err(|_| SqlError::InvalidTable)?;

        let reader = BufReader::new(file);

        Ok(reader)
    }
}
