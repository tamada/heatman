use crate::cli::Heatman;
use clap::Parser;

use std::path::Path;
use heatman::{Heatmap, Error, Data, Result};
use image::{ImageBuffer, Rgba};

mod cli;

fn output_image(data: Data<Rgba<u8>>, pixel: usize, dest: &Path) -> Result<()> {
    let context = Heatmap::new(data, pixel);
    let image: ImageBuffer<Rgba<u8>, Vec<u8>> = context.into();
    image.save(dest)
        .map_err(Error::Image)
}

fn generate_scaler(heatman: Heatman) -> Result<()> {
    let data = heatman::ScalerBuilder::build();
    let rgbdata: Data<Rgba<u8>> = data.into();
    output_image(rgbdata, heatman.pixel(), heatman.dest())
}

fn generate_heatmap(heatman: Heatman) -> Result<()> {
    let data = heatman.load_image()?;
    let order = heatman.order(&data)?;
    let order = order.apply_assistant_line(heatman.assistant_line_gap());
    let reordered_data = data.reorder(&order);
    let rgbdata: Data<Rgba<u8>> = reordered_data.into();
    output_image(rgbdata, heatman.pixel(), heatman.dest())
}

fn generate_items<F>(heatman: Heatman, mapper: F) -> Result<()>
where
        F: Fn(&Data<f64>) -> Vec<String> 
{
    let data = heatman.load_image()?;
    let order = heatman.order(&data)?;
    let order = order.apply_assistant_line(heatman.assistant_line_gap());
    let reordered_data = data.reorder(&order);
    let items = mapper(&reordered_data);
    for item in items {
        println!("{item}");
    }
    Ok(())
}

#[cfg(debug_assertions)]
mod gencomp;

fn rs_main(args: Vec<String>) -> Result<()> {
    let heatman: Heatman = Parser::try_parse_from(args)
        .map_err(Error::Clap)?;
    let heatman = heatman.validate()?;
    match heatman.mode() {
        cli::Mode::Scaler => generate_scaler(heatman),
        cli::Mode::Heatmap => generate_heatmap(heatman),
        cli::Mode::Rows => generate_items(heatman, |data| data.row_headers()),
        cli::Mode::Columns => generate_items(heatman, |data| data.col_headers()),
        #[cfg(debug_assertions)]
        cli::Mode::GenerateCompletions => {
            gencomp::generate("heatman", Path::new("completions"));
            Ok(())
        },
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    match rs_main(args) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}