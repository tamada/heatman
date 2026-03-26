use std::path::Path;
use std::io::{BufRead, BufReader};

use image::{ImageBuffer, Rgba};

use crate::{Data, Error, Result};

/// Context for generating heatmaps, containing the data and pixel size.
pub struct Heatmap<T> {
    pixel: usize,
    data: Data<T>,
}

impl<T> Heatmap<T> {
    /// Creates a new Context with the given data and pixel size.
    /// The pixel size determines how many pixels each cell in the heatmap will occupy.
    pub fn new(data: Data<T>, pixel: usize) -> Self {
        Heatmap { data, pixel }
    }

    /// Calculates the total height of the output image in pixels.
    pub fn image_height(&self) -> usize {
        self.data.pixel_mapping_row(self.pixel).len()
    }

    /// Calculates the total width of the output image in pixels.
    pub fn image_width(&self) -> usize {
        self.data.pixel_mapping_col(self.pixel).len()
    }
}

impl Heatmap<f64> {
    pub fn to_image(self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let (pixel, data) = (self.pixel, self.data);
        let data: Data<Rgba<u8>> = data.into();
        let context = Heatmap { pixel, data, };
        context.to_image()
    }
}

impl Heatmap<Rgba<u8>> {
    pub fn to_image(self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let (pixel, data) = (self.pixel, self.data);
        let context = Heatmap { pixel, data, };
        context.into()
    }
}

impl From<Heatmap<f64>> for ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn from(map: Heatmap<f64>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let data: Data<Rgba<u8>> = map.data.convert_with(crate::convert);
        let context = Heatmap {
            pixel: map.pixel,
            data,
        };
        context.into()
    }
}

impl From<Heatmap<Rgba<u8>>> for ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn from(map: Heatmap<Rgba<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let img_width = map.image_width() as u32;
        let img_height = map.image_height() as u32;
        log::info!("Output image size: {}x{}", img_width, img_height);
        let gap_color = Rgba([255, 255, 255, 0]);
        let mut result_image = ImageBuffer::new(img_width, img_height);

        let row_mapping = map.data.pixel_mapping_row(map.pixel);
        let col_mapping = map.data.pixel_mapping_col(map.pixel);

        for (py, row_index) in row_mapping.into_iter().enumerate() {
            for (px, col_index) in col_mapping.iter().enumerate() {
                let color = match (row_index, *col_index) {
                    (Some(ri), Some(ci)) => 
                        map.data.cell(ri, ci)
                            .copied().unwrap_or(gap_color),
                    _ => gap_color,
                };
                result_image.put_pixel(px as u32, py as u32, color);
            }
        }
        result_image
    }
}

/// Represents the order of rows and columns in a heatmap.
pub enum Order {
    /// Symmetric order where rows and columns are the same.
    Symmetric(Vec<String>),
    /// Asymmetric order where rows and columns are different.
    Asymmetric(Vec<String>, Vec<String>),
}

impl Order {
    /// Loads a symmetric order from the specified path.
    pub fn load_symmetric<P: AsRef<Path>>(path: P) -> Result<Self> {
        load_order_vec(path)
            .map(Order::Symmetric)
    }

    /// Loads an asymmetric order from the specified row and column paths.
    pub fn load_asymmetric<P: AsRef<Path>>(row_path: P, column_path: P) -> Result<Self> {
        let rows = load_order_vec(row_path)?;
        let cols = load_order_vec(column_path)?;
        Ok(Order::Asymmetric(rows, cols))
    }

    /// Returns true if the row order is empty.
    pub fn is_row_empty(&self) -> bool {
        match self {
            Order::Symmetric(items) => items.is_empty(),
            Order::Asymmetric(rows, _) => rows.is_empty(),
        }
    }

    /// Returns true if the column order is empty.
    pub fn is_column_empty(&self) -> bool {
        match self {
            Order::Symmetric(items) => items.is_empty(),
            Order::Asymmetric(_, cols) => cols.is_empty(),
        }
    }

    /// Returns an iterator over the row names.
    pub fn rows(&self) -> impl Iterator<Item = &String> {
        match self {
            Order::Symmetric(items) => items.iter(),
            Order::Asymmetric(rows, _) => rows.iter(),
        }
    }

    /// Returns an iterator over the column names.
    pub fn columns(&self) -> impl Iterator<Item = &String> {
        match self {
            Order::Symmetric(items) => items.iter(),
            Order::Asymmetric(_, cols) => cols.iter(),
        }
    }

    /// Applies assistant lines to the order at the specified gap interval.
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
        .map_err(Error::Io)?;
    let reader = BufReader::new(file);
    let mut items = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(Error::Io)
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
