//! Heatman is for generating heatmaps from tabular data.
//! The library provides core functionality for data processing and heatmap generation, while the CLI handles user input and interaction.
//! 
//! ## Examples
//! 
//! ### Example 1: Generating a heatmap from a CSV file
//! 
//! The simple use case available on `examples/generate_heatmap_from_csv.rs`.
//! 
//! ```rust
//! use heatman::{Heatmap, Data, DataLoader};
//! use image::{ImageBuffer, Rgba};
//! 
//! let data = DataLoader::load("testdata/sample.csv") // load data from CSV file
//!     .expect("Failed to load data from CSV");
//! let context = Heatmap::new(cells, 10);       // create heatmap context with pixel size of 10
//! let image: ImageBuffer<Rgba<u8>, Vec<u8>> = context.into(); // convert heatmap context into an image
//! image.save("example1.png")                   // save the generated heatmap image
//!     .expect("Failed to save heatmap image");
//! ```
//! 
//! ### Example 2: Generating a scaler
//! 
//! Generate a scaler image with height of 20 pixels, which can be used as a reference for interpreting the heatmap colors.
//! The example is available on `examples/generate_scaler.rs`.
//! 
//! ```rust
//! use heatman::{Heatmap, ScalerBuilder};
//! use image::{ImageBuffer, Rgba};
//! 
//! let data = ScalerBuilder::build_with(20);   // create scaler data with height of 20 pixels
//! let context = Heatmap::new(data, 1);        // create heatmap context with pixel size of 1 (each cell corresponds to 1 pixel)
//! let image: ImageBuffer<Rgba<u8>, Vec<u8>> = context.into();
//! image.save("scaler.png")                    // save the generated scaler image
//!     .expect("Failed to save scaler image");
//! ```
//! 
//! ### Example 3: Specify the order of rows and columns
//! 
//! ```rust
//! let data = DataLoader::load("testdata/jaccard.csv")
//!     .expect("Failed to load data from CSV");
//! let order = Order::load_symmetric("testdata/header.csv")
//!     .expect("Failed to load order from file");
//! let reordered_data = data.reorder(&order);
//! let cells: Data<Rgba<u8>> = reordered_data.into();
//! let context = Heatmap::new(cells, 10);
//! let image: ImageBuffer<Rgba<u8>, Vec<u8>> = context.into();
//! image.save("jaccard.png")
//!     .expect("Failed to save heatmap image");
//! ```
mod data;
mod heatmap;

pub use data::{Data, DataLoader, convert};
pub use heatmap::{Heatmap, Order};
use std::{fmt::Display, path::PathBuf};

/// A specialized Result type for heatman operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents errors that can occur during heatman operations.
#[derive(Debug)]
pub enum Error {
    /// Multiple errors.
    Array(Vec<Error>),
    /// Error from clap.
    Clap(clap::Error),
    /// Error from csv.
    Csv(csv::Error),
    /// File not found.
    FileNotFound(PathBuf),
    /// Error from image.
    Image(image::ImageError),
    /// Invalid data.
    InvalidData(String),
    /// IO error.
    Io(std::io::Error),
    /// Error parsing value range.
    ParseRange(String, String),
    /// Error parsing float.
    ParseFloat(String, std::num::ParseFloatError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Array(errs) => {
                for (i, err) in errs.iter().enumerate() {
                    writeln!(f, "Error {}: {}", i + 1, err)?;
                }
                Ok(())
            },
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::ParseRange(s, msg) => write!(f, "{s}: Failed to parse range: {}", msg),
            Error::ParseFloat(s, e) => write!(f, "{s}: Failed to parse float: {}", e),
            Error::Clap(e) => write!(f, "Command line argument error: {}", e),
            Error::Image(e) => write!(f, "Image error: {}", e),
            Error::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            Error::FileNotFound(path) => write!(f, "{}: File not found", path.display()),
            Error::Csv(e) => write!(f, "CSV error: {}", e),
        }
    }
}

/// Generates a scaler image data with the specified height.
pub struct ScalerBuilder;

impl ScalerBuilder {
    /// Creates a new `Data` instance with a scaler image with height is 5.
    /// This method is same as `Data::scaler_with(5)`.
    pub fn build() -> Data<f64> {
        Self::build_with(5)
    }

    /// Creates a new `Data` instance with a scaler image of the specified height.
    pub fn build_with(height: usize) -> Data<f64> {
        let range = 0_i32..240_i32;
        let line = range.into_iter()
            .map(|i| Some(i as f64 / 240.0))
            .collect::<Vec<Option<f64>>>();
        let table = (0..height)
            .map(|_| line.clone()).collect::<Vec<Vec<_>>>();
        Data::new(table)
    }
}