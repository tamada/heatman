use std::{ops::RangeInclusive, path::Path};
use image::Rgba;
use palette::{Hsv, IntoColor, Srgb};

use crate::{Error, Result};

/// Represents a matrix of data with optional row and column headers.
pub struct Data<T> {
    row_headers: Headers,
    col_headers: Headers,
    cells: Vec<Vec<Option<T>>>,
}

impl<T> Data<T> {
    /// Creates a new `Data` instance without headers.
    pub fn new(cells: Vec<Vec<Option<T>>>) -> Self {
        Self::new_with_headers(vec![], vec![], cells)
    }

    /// Creates a new `Data` instance with specified row and column headers.
    pub fn new_with_headers(row_headers: Vec<String>, col_headers: Vec<String>, cells: Vec<Vec<Option<T>>>) -> Self {
        Data { row_headers: Headers { items: row_headers }, col_headers: Headers { items: col_headers }, cells }
    }

    /// Generates a mapping from pixel rows to data row indices, considering the pixel size and assistant lines.
    /// Returns a vector where each element is `Some(index)` of the data row, or `None` for a gap (assistant line).
    pub fn pixel_mapping_row(&self, pixel_size: usize) -> Vec<Option<usize>> {
        self.row_headers.pixel_mapping(self.rows(), pixel_size)
    }

    /// Generates a mapping from pixel columns to data column indices, considering the pixel size and assistant lines.
    /// Returns a vector where each element is `Some(index)` of the data column, or `None` for a gap (assistant line).
    pub fn pixel_mapping_col(&self, pixel_size: usize) -> Vec<Option<usize>> {
        self.col_headers.pixel_mapping(self.cols(), pixel_size)
    }

    /// Calculates the total height of the output image in pixels.
    pub fn image_height(&self, pixel_size: usize) -> usize {
        self.pixel_mapping_row(pixel_size).len()
    }

    /// Calculates the total width of the output image in pixels.
    pub fn image_width(&self, pixel_size: usize) -> usize {
        self.pixel_mapping_col(pixel_size).len()
    }

    /// Returns the number of logical rows in the data.
    pub fn rows(&self) -> usize {
        if self.row_headers.is_empty() {
            self.cells.len()
        } else {
            self.row_headers.items.iter().filter(|&h| !is_assistant_line(h)).count()
        } 
    }

    /// Returns the number of logical columns in the data.
    pub fn cols(&self) -> usize {
        if self.col_headers.is_empty() {
            self.cells.iter()
                .map(|row| row.len())
                .max().unwrap_or(0)
        } else {
            self.col_headers.items.iter().filter(|&h| !is_assistant_line(h)).count()
        }
    }

    pub fn row_header(&self, index: usize) -> Option<&String> {
        self.row_headers.get(index)
    }

    pub fn col_header(&self, index: usize) -> Option<&String> {
        self.col_headers.get(index)
    }

    pub fn row_headers(&self) -> Vec<String> {
        if self.row_headers.is_empty() {
            (0..self.rows()).map(|i| i.to_string()).collect()
        } else {
            self.row_headers.items.clone()
        }
    }

    pub fn col_headers(&self) -> Vec<String> {
        if self.col_headers.is_empty() {
            (0..self.cols()).map(|i| i.to_string()).collect()
        } else {
            self.col_headers.items.clone()
        }
    }

    pub fn cell(&self, row: usize, col: usize) -> Option<&T> {
        get_cell(&self.cells, row, col)
    }

    pub fn cell_symmetric(&self, row: usize, col: usize) -> Option<&T> {
        get_cell(&self.cells, row, col)
            .or_else(|| get_cell(&self.cells, col, row))
    }

    pub fn cell_of(&self, row_name: &str, col_name: &str) -> Option<&T> {
        get_cell_of(self, row_name, col_name)
    }

    pub fn cell_of_symmetric(&self, row_name: &str, col_name: &str) -> Option<&T> {
        get_cell_of(self, row_name, col_name)
            .or_else(|| get_cell_of(self, col_name, row_name))
    }

    pub fn convert<U: From<T>>(self) -> Data<U> {
        let cells = self.cells.into_iter()
            .map(|row| row.into_iter()
                .map(|cell| cell.and_then(|v| Some(v.into())))
                .collect::<Vec<Option<U>>>())
            .collect::<Vec<Vec<Option<U>>>>();
        Data { row_headers: self.row_headers, col_headers: self.col_headers, cells }
    }

    pub fn convert_with<U, F: Fn(T) -> U>(self, f: F) -> Data<U> {
        let cells = self.cells.into_iter()
            .map(|row| row.into_iter()
                .map(|cell| cell.and_then(|v| Some(f(v))))
                .collect::<Vec<Option<U>>>())
            .collect::<Vec<Vec<Option<U>>>>();
        Data { row_headers: self.row_headers, col_headers: self.col_headers, cells }
    }

    pub fn is_lower_triangular(&self) -> bool {
        let rows = self.rows();
        let cols = self.cols();
        if rows != cols {
            return false;
        }
        for r in 0..rows {
            for c in r + 1..cols {
                if get_cell(&self.cells, r, c).is_some() {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_upper_triangular(&self) -> bool {
        let rows = self.rows();
        let cols = self.cols();
        if rows != cols {
            return false;
        }
        for r in 0..rows {
            for c in 0..r {
                if get_cell(&self.cells, r, c).is_some() {
                    return false;
                }
            }
        }
        true
    }

    pub fn reorder(self, order: &crate::Order) -> Self 
        where T: Clone
    {
        let is_lower = self.is_lower_triangular();
        let is_upper = self.is_upper_triangular();
        let is_symmetric = matches!(order, crate::Order::Symmetric(_));

        let row_headers: Vec<String> = order.rows().cloned().collect();
        let col_headers: Vec<String> = order.columns().cloned().collect();

        let mut new_cells = Vec::new();
        let filtered_rows: Vec<_> = row_headers.iter().filter(|&h| !is_assistant_line(h)).collect();
        let filtered_cols: Vec<_> = col_headers.iter().filter(|&h| !is_assistant_line(h)).collect();

        for (i, row_name) in filtered_rows.into_iter().enumerate() {
            let mut new_row = Vec::new();
            for (j, col_name) in filtered_cols.iter().enumerate() {
                let cell = if is_symmetric {
                    let value = self.cell_of_symmetric(row_name, col_name).cloned();
                    if is_lower && i < j {
                        None
                    } else if is_upper && i > j {
                        None
                    } else {
                        value
                    }
                } else {
                    self.cell_of(row_name, col_name).cloned()
                };
                new_row.push(cell);
            }
            new_cells.push(new_row);
        }
        Data {
            row_headers: Headers { items: row_headers },
            col_headers: Headers { items: col_headers },
            cells: new_cells,
        }
    }
}

impl From<Data<f64>> for Data<Rgba<u8>> {
    fn from(data: Data<f64>) -> Self {
        data.convert_with(convert)
    }
}

pub fn convert(value: f64) -> Rgba<u8> {
    let r = value.clamp(0.0, 1.0);
    let hue = (1.0 - r) * 240.0; // 0 (red) to 240 (blue)
    let hsv = Hsv::new(hue, 1.0, 1.0);
    let rgbf: Srgb<f64> = hsv.into_color();
    let rgb = rgbf.into_format::<u8>();
    Rgba([rgb.red, rgb.green, rgb.blue, 255])
}

fn get_cell<T>(cells: &Vec<Vec<Option<T>>>, row: usize, col: usize) -> Option<&T> {
    cells.get(row)?.get(col)?.as_ref()
}

fn get_cell_of<'a, T>(data: &'a Data<T>, row_name: &str, col_name: &str) -> Option<&'a T> {
    let row_index = data.row_headers.index_of(row_name)?;
    let col_index = data.col_headers.index_of(col_name)?;
    data.cells.get(row_index)?.get(col_index)?.as_ref()
}

/// Manages a list of headers, including support for assistant line markers ("---").
pub struct Headers {
    items: Vec<String>,
}

impl Headers {
    /// Creates a pixel-to-index mapping for the headers.
    /// 
    /// If headers are empty, it generates a simple repeated mapping based on `count` and `pixel_size`.
    /// If headers exist, it respects the "---" markers to insert gaps (`None`) into the mapping.
    pub fn pixel_mapping(&self, count: usize, pixel_size: usize) -> Vec<Option<usize>> {
        if self.is_empty() {
            let mut mapping = Vec::new();
            for i in 0..count {
                for _ in 0..pixel_size {
                    mapping.push(Some(i));
                }
            }
            mapping
        } else {
            let mut mapping = Vec::new();
            let mut current_index = 0;
            for item in self.items.iter() {
                if is_assistant_line(item) {
                    mapping.push(None); // Assistant line is strictly 1 pixel
                } else {
                    for _ in 0..pixel_size {
                        mapping.push(Some(current_index));
                    }
                    current_index += 1;
                }
            }
            mapping
        }
    }

    /// Returns true if there are no headers defined.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Gets the header item at the specified index.
    pub fn get(&self, index: usize) -> Option<&String> {
        self.items.get(index)
    }

    /// Returns the index of the header item with the specified name, ignoring assistant lines.
    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.items.iter()
            .filter(|&item| !is_assistant_line(item))
            .position(|item| item == name)
    }
}

pub fn is_assistant_line(s: &str) -> bool {
    s.len() >= 3 && s.chars().all(|c| c == '-')
}

pub fn load_with<P: AsRef<Path>>(path: P, range: &RangeInclusive<f64>) -> Result<Data<f64>> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(Error::FileNotFound(path.to_path_buf()));
    }
    let mut rdr = csv::Reader::from_path(path)
        .map_err(Error::Csv)?;
    let col_headers = Headers {
        items: rdr.headers().map_err(Error::Csv)?
            .iter().skip(1).map(|s| s.to_string()).collect(),
    };
    let mut row_headers = Vec::new();
    let mut cells = Vec::new();
    let mut errs = Vec::new();
    for result in rdr.records() {
        let record = match result {
            Ok(rec) => rec,
            Err(e) => {
                errs.push(Error::Csv(e));
                continue;
            }
        };
        row_headers.push(record.get(0).unwrap_or("").to_string());
        let row = record.iter().skip(1)
            .map(|s| {
                let value = match s.parse::<f64>() {
                    Ok(v) => v,
                    Err(e) => {
                        errs.push(Error::ParseFloat(s.to_string(), e));
                        return None;
                    }
                };
                Some(clamp_and_scale(value, &range))
            }).collect();
        cells.push(row);
    }
    Ok(Data { row_headers: Headers { items: row_headers }, col_headers, cells })
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<Data<f64>> {
    load_with(path, &(0.0..=1.0))
}

/// Normalize the value to the range [0.0, 1.0], clamping values outside the range to the nearest bound.
pub fn clamp_and_scale(value: f64, range: &RangeInclusive<f64>) -> f64 {
    let start = *range.start();
    let end = *range.end();
    if value < start {
        0.0
    } else if value > end {
        1.0
    } else {
        (value - start) / (end - start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headers_index_of() {
        let headers = Headers {
            items: vec!["A".to_string(), "---".to_string(), "B".to_string(), "----".to_string()],
        };
        assert_eq!(headers.index_of("A"), Some(0));
        assert_eq!(headers.index_of("B"), Some(1));
        assert_eq!(headers.index_of("---"), None);
        assert_eq!(headers.index_of("----"), None);
    }

    #[test]
    fn test_asymmetric_reordering_preserves_triangularity() {
        let cells = vec![
            vec![Some(1.0), None],
            vec![Some(2.0), Some(3.0)],
        ];
        let headers = vec!["A".to_string(), "B".to_string()];
        let data = Data::new_with_headers(headers.clone(), headers.clone(), cells);
        
        let order = crate::Order::Asymmetric(headers.clone(), headers.clone());
        let reordered = data.reorder(&order);
        
        // it should still be lower triangular
        assert!(reordered.is_lower_triangular());
        assert_eq!(reordered.cell(0, 1), None);
    }

    #[test]
    fn test_symmetric_reordering_of_triangular() {
        let cells = vec![
            vec![Some(1.0), None],
            vec![Some(2.0), Some(3.0)],
        ];
        let headers = vec!["A".to_string(), "B".to_string()];
        let data = Data::new_with_headers(headers.clone(), headers.clone(), cells);
        
        // Reverse order
        let mut reversed = headers.clone();
        reversed.reverse();
        let order = crate::Order::Symmetric(reversed);
        let reordered = data.reorder(&order);
        
        // Original: (A,A)=1.0, (A,B)=None, (B,A)=2.0, (B,B)=3.0
        // New headers: [B, A]
        // (B,B)=3.0, (B,A)=2.0, (A,B)=None (but symmetric lookup finds 2.0?), (A,A)=1.0
        // wait, reorder with Symmetric should force it to be lower triangular
        // New order (B, A)
        // (0,0) is (B,B)=3.0.
        // (1,0) is (A,B). cell_of_symmetric(A,B) is Some(2.0).
        // (0,1) is (B,A). cell_of_symmetric(B,A) is Some(2.0). BUT i < j, so it sets None.
        // (1,1) is (A,A)=1.0.
        // So the new matrix is:
        // [[3.0, None],
        //  [2.0, 1.0]]
        // This is STILL lower triangular!
        assert!(reordered.is_lower_triangular());
        assert_eq!(reordered.cell(0, 1), None);
        assert_eq!(reordered.cell(1, 0), Some(&2.0));
    }

    #[test]
    fn test_triangular_preservation() {
        // Create a simple lower triangular matrix
        let cells = vec![
            vec![Some(1.0), None, None],
            vec![Some(2.0), Some(3.0), None],
            vec![Some(4.0), Some(5.0), Some(6.0)],
        ];
        let headers = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let data = Data::new_with_headers(headers.clone(), headers.clone(), cells);
        assert!(data.is_lower_triangular());
        assert!(!data.is_upper_triangular());

        // cell(0, 1) should be None if we want to preserve triangularity
        assert_eq!(data.cell(0, 1), None);
    }

    #[test]
    fn test_triangular_with_assistant_lines() {
        let cells = vec![
            vec![Some(1.0), None],
            vec![Some(2.0), Some(3.0)],
        ];
        let headers = vec!["A".to_string(), "---".to_string(), "B".to_string()];
        let data = Data::new_with_headers(headers.clone(), headers.clone(), cells);
        
        // rows() and cols() should be 2
        assert_eq!(data.rows(), 2);
        assert_eq!(data.cols(), 2);
        
        // it should still be lower triangular
        assert!(data.is_lower_triangular());
    }
}
