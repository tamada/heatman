use std::path::Path;
use std::io::{BufRead, BufReader};

use crate::{Error, Result};

pub struct Heatmap {

}

pub enum Order {
    Symmetric(Vec<String>),
    Asymmetric(Vec<String>, Vec<String>),
}

impl Order {
    pub fn load_symmetric<P: AsRef<Path>>(path: P) -> Result<Self> {
        load_order_vec(path)
            .map(Order::Symmetric)
    }

    pub fn load_asymmetric<P: AsRef<Path>>(row_path: P, column_path: P) -> Result<Self> {
        let rows = load_order_vec(row_path)?;
        let cols = load_order_vec(column_path)?;
        Ok(Order::Asymmetric(rows, cols))
    }

    pub fn is_row_empty(&self) -> bool {
        match self {
            Order::Symmetric(items) => items.is_empty(),
            Order::Asymmetric(rows, _) => rows.is_empty(),
        }
    }

    pub fn is_column_empty(&self) -> bool {
        match self {
            Order::Symmetric(items) => items.is_empty(),
            Order::Asymmetric(_, cols) => cols.is_empty(),
        }
    }

    pub fn rows(&self) -> impl Iterator<Item = &String> {
        match self {
            Order::Symmetric(items) => items.iter(),
            Order::Asymmetric(rows, _) => rows.iter(),
        }
    }

    pub fn columns(&self) -> impl Iterator<Item = &String> {
        match self {
            Order::Symmetric(items) => items.iter(),
            Order::Asymmetric(_, cols) => cols.iter(),
        }
    }

    pub fn apply_assistant_line(&mut self, gap: usize) {
        if gap == 0 {
            return;
        }
        match self {
            Order::Symmetric(items) => {
                *items = insert_assistant_lines(items.clone(), gap);
            },
            Order::Asymmetric(rows, cols) => {
                *rows = insert_assistant_lines(rows.clone(), gap);
                *cols = insert_assistant_lines(cols.clone(), gap);
            }
        }
    }
}

fn insert_assistant_lines(items: Vec<String>, gap: usize) -> Vec<String> {
    let mut result = Vec::new();
    for (i, item) in items.into_iter().enumerate() {
        if i > 0 && i % gap == 0 {
            result.push("---".to_string());
        }
        result.push(item);
    }
    result
}

fn load_order_vec<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let file = std::fs::File::open(path)
        .map_err(|e| Error::Io(e))?;
    let reader = BufReader::new(file);
    let mut items = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(|e| Error::Io(e))
            .map(strip_comment)?;
        items.push(line.to_string());
    }
    Ok(items)
}

fn strip_comment(line: String) -> String {
    let mut result = String::with_capacity(line.len());
    let mut escaped = false;
    for c in line.chars() {
        if escaped {
            result.push(c);
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else if c == '#' {
            break;
        } else {
            result.push(c);
        }
    }
    result.trim().to_string()
}