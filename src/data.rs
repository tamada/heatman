use std::{ops::RangeInclusive, path::Path};
use image::Rgba;
use palette::{Hsv, IntoColor, Srgb};

use crate::{Error, Result};

pub struct Data<T> {
    row_headers: Headers,
    col_headers: Headers,
    cells: Vec<Vec<Option<T>>>,
}

impl<T> Data<T> {
    pub fn new(cells: Vec<Vec<Option<T>>>) -> Self {
        Self::new_with_headers(vec![], vec![], cells)
    }

    pub fn new_with_headers(row_headers: Vec<String>, col_headers: Vec<String>, cells: Vec<Vec<Option<T>>>) -> Self {
        Data { row_headers: Headers { items: row_headers }, col_headers: Headers { items: col_headers }, cells }
    }

    pub fn gapps_row(&self, pixel_size: usize) -> Vec<bool> {
        self.row_headers.gaps(pixel_size)
    }

    pub fn gapps_col(&self, pixel_size: usize) -> Vec<bool> {
        self.col_headers.gaps(pixel_size)
    }

    pub fn image_height(&self, pixel_size: usize) -> usize {
        if self.row_headers.is_empty() {
            self.rows() * pixel_size
        } else {
            self.row_headers.gaps(pixel_size).len()
        }
    }

    pub fn image_width(&self, pixel_size: usize) -> usize {
        if self.col_headers.is_empty() {
            self.cols() * pixel_size
        } else {
            self.col_headers.gaps(pixel_size).len()
        }
    }

    pub fn rows(&self) -> usize {
        if self.row_headers.is_empty() {
            self.cells.len()
        } else {
            self.row_headers.len()
        } 
    }

    pub fn cols(&self) -> usize {
        if self.col_headers.is_empty() {
            self.cells.iter()
                .map(|row| row.len())
                .max().unwrap_or(0)
        } else {
            self.col_headers.len()
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
        if let Some(item) = get_cell(&self.cells, row, col) {
            Some(item)
        } else {
            get_cell(&self.cells, col, row)
        }
    }

    pub fn cell_of(&self, row_name: &str, col_name: &str) -> Option<&T> {
        if let Some(item) = get_cell_of(self, row_name, col_name) {
            Some(item)
        } else {
            get_cell_of(self, col_name, row_name)
        }
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

pub struct Headers {
    items: Vec<String>,
}

impl Headers {
    pub fn gaps(&self, pixel_size: usize) -> Vec<bool> {
        let mut gaps = Vec::new();
        for item in self.items.iter() {
            if item == "---" {
                gaps.push(true);
            } else {
                for _i in 0..pixel_size {
                    gaps.push(false);
                }
            }
        }
        gaps
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, index: usize) -> Option<&String> {
        self.items.get(index)
    }

    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.items.iter().position(|item| item == name)
    }
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
