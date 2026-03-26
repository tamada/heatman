use std::{ops::RangeInclusive, path::{Path, PathBuf}};

use clap::{Parser, ValueEnum};
use heatman::{Data, Error, Order, Result};

/// Output mode for the heatman CLI.
#[derive(Debug, Parser, ValueEnum, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    /// Output a scaler image instead of a heatmap. In this mode, the pixel option will be used as the height of the scaler image.
    Scaler,
    /// Output a heatmap image.
    Heatmap,
    /// Output the rows of the data.
    Rows,
    /// Output the columns of the data.
    Columns,
    #[cfg(debug_assertions)]
    /// Generate shell completion scripts for the CLI. In this mode, almost options will be ignored and output the completion scripts into the directory specified by dest option.
    GenerateCompletions,
}

/// Command line arguments for the heatman CLI.
#[derive(Debug, Parser)]
#[command(author, version, about = "Heatmap generator for visualizing data in a matrix format.")]
pub struct Heatman {
    #[clap(short, long, default_value = "heatman.png", help = "Destination path for the output image")]
    dest: PathBuf,

    #[clap(short, long, default_value_t = 0, help = "Assistant line gap in cells. If 0, no assistant line will be drawn.")]
    assistant_line_gap: usize,

    #[clap(short, long, default_value_t = 3, help = "Pixel size of of cells")]
    pixel: usize,

    #[clap(short, long, value_enum, default_value_t = Mode::Heatmap, help = "Output mode")]
    mode: Mode,

    #[clap(short, long, default_value = "warn", value_enum, help = "Logging level (trace, debug, info, warn, error)")]
    level: LogLevel,

    #[clap(short, long, default_value = "0-1", help = "Specify the value range for the input data.", value_parser = parse_range)]
    range: RangeInclusive<f64>,

    #[clap(long, conflicts_with_all = ["row_order", "column_order"],
        help = "The file describing the order of the data to be plotted.
Each line should contain a single string that matches the name of the data to be plotted.
The triple dash line (---) means the assistant line is drawin here.
The # is the comment line. escape the # with \\# if you want to use it as a name.
If not provided, the order will be determined by the order of the data in the input files.")]
    order: Option<PathBuf>,

    #[clap(long, help = "The file describing the order of the rows to be plotted.", requires = "column_order")]
    row_order: Option<PathBuf>,

    #[clap(long, help = "The file describing the order of the columns to be plotted.", requires = "row_order")]
    column_order: Option<PathBuf>,

    #[clap(index = 1, help = "Input files containing the data to be plotted. The csv file should contains the row and column header.")]
    input_file: Option<PathBuf>,
}

/// Logging levels for the heatman CLI.
#[derive(Debug, Clone, ValueEnum)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

fn parse_range(s: &str) -> std::result::Result<RangeInclusive<f64>, String> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 2 {
        return Err(format!("{s}: The range should be specified as '10-50' with a hyphen"));
    }
    
    let start = parts[0].parse::<f64>()
        .map_err(|e| format!("{s}: Failed to parse start value: {}", e))?;
    let end = parts[1].parse::<f64>()
        .map_err(|e| format!("{s}: Failed to parse end value: {}", e))?;
    
    if start > end {
        return Err(format!("{s}: Start value must be less than or equal to end value"));
    }
    
    Ok(start..=end)
}

fn init_log(level: &LogLevel) {
    unsafe {
        match level {
            LogLevel::Trace => std::env::set_var("RUST_LOG", "trace"),
            LogLevel::Debug => std::env::set_var("RUST_LOG", "debug"),
            LogLevel::Info => std::env::set_var("RUST_LOG", "info"),
            LogLevel::Warn => std::env::set_var("RUST_LOG", "warn"),
            LogLevel::Error => std::env::set_var("RUST_LOG", "error"),
        }
    };
    env_logger::init()
}

fn input_file_required(h: Heatman) -> Result<Heatman> {
    if h.input_file.is_none() {
        Err(Error::InvalidData("Input file must be specified in heatmap mode".to_string()))
    } else if let Some(path) = &h.input_file && !path.exists() {
        Err(Error::FileNotFound(path.clone()))
    } else {
        Ok(h)
    }
}

impl Heatman {
    /// Validates the command line arguments.
    pub fn validate(self) -> Result<Self> {
        init_log(&self.level);
        match self.mode {
            Mode::Scaler => {
                if self.input_file.is_some() {
                    Err(Error::InvalidData("Input file should not be specified in scaler mode".to_string()))
                } else {
                    Ok(self)
                }
            },
            Mode::Heatmap => input_file_required(self),
            Mode::Rows | Mode::Columns => input_file_required(self),
            #[cfg(debug_assertions)]
            Mode::GenerateCompletions => Ok(self),  
        }
    }

    /// Loads the data from the input file.
    pub fn load_image(&self) -> Result<heatman::Data<f64>> {
        heatman::DataLoader::with(self.input_file.as_ref().unwrap(), &self.range)
    }

    /// Determines the order of rows and columns.
    pub fn order<T>(&self, data: &Data<T>) -> Result<Order> {
        if let Some(order_path) = &self.order {
            Order::load_symmetric(order_path)
        } else if let (Some(row_order_path), Some(col_order_path)) = (&self.row_order, &self.column_order) {
            Order::load_asymmetric(row_order_path, col_order_path)
        } else {
            Ok(Order::Asymmetric(data.row_headers(), data.col_headers()))
        }
    }

    /// Returns the pixel size of cells.
    pub fn pixel(&self) -> usize {
        self.pixel
    }

    /// Returns the destination path for the output image.
    pub fn dest(&self) -> &Path {
        &self.dest
    }

    /// Returns the output mode.
    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    /// Returns the assistant line gap.
    pub fn assistant_line_gap(&self) -> usize {
        self.assistant_line_gap
    }
}
