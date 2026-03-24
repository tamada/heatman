mod data;
mod heatmap;

pub use data::{Data, convert, load, load_with};
pub use heatmap::{Heatmap, Order};
use std::{fmt::Display, path::PathBuf};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Array(Vec<Error>),
    Clap(clap::Error),
    Csv(csv::Error),
    FileNotFound(PathBuf),
    Image(image::ImageError),
    InvalidData(String),
    Io(std::io::Error),
    ParseRange(String, String),
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
