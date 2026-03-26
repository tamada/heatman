use crate::cli::Heatman;
use clap::Parser;

use std::path::Path;
use heatman::{Error, Data, Result};
use image::{ImageBuffer, Rgba};

mod cli;

fn output_image(data: Data<Rgba<u8>>, pixel_width: usize, pixel_height: usize, dest: &Path) -> Result<()> {
    let img_width = data.image_width(pixel_width) as u32;
    let img_height = data.image_height(pixel_height) as u32;
    log::info!("Output image size: {}x{}", img_width, img_height);
    let gap_color = Rgba([255, 255, 255, 0]);
    let mut result_image = ImageBuffer::new(img_width, img_height);

    let row_mapping = data.pixel_mapping_row(pixel_height);
    let col_mapping = data.pixel_mapping_col(pixel_width);

    for (py, row_index) in row_mapping.into_iter().enumerate() {
        for (px, col_index) in col_mapping.iter().enumerate() {
            let color = match (row_index, *col_index) {
                (Some(ri), Some(ci)) => data.cell(ri, ci).map(|c| *c).unwrap_or(gap_color),
                _ => gap_color,
            };
            result_image.put_pixel(px as u32, py as u32, color);
        }
    }

    result_image.save(dest)
        .map_err(|e| Error::Image(e))
}

fn generate_scaler(heatman: Heatman) -> Result<()> {
    let range = 0_i32..240_i32;
    let line = range.into_iter()
        .map(|i| Some(i as f64 / 240.0))
        .collect::<Vec<Option<f64>>>();
    let table = (0..10)
        .map(|_| line.clone()).collect::<Vec<Vec<_>>>();
    let data = Data::new(table);
    let rgbdata: Data<Rgba<u8>> = data.into();
    output_image(rgbdata, 1, heatman.pixel(), heatman.dest())
}

fn generate_heatmap(heatman: Heatman) -> Result<()> {
    let data = heatman.load_image()?;
    let mut order = heatman.order(&data)?;
    order.apply_assistant_line(heatman.assistant_line_gap());
    let reordered_data = data.reorder(&order);
    let rgbdata: Data<Rgba<u8>> = reordered_data.into();
    output_image(rgbdata, heatman.pixel(), heatman.pixel(), heatman.dest())
}

fn generate_items<F>(heatman: Heatman, mapper: F) -> Result<()>
where
        F: Fn(&Data<f64>) -> Vec<String> 
{
    let data = heatman.load_image()?;
    let mut order = heatman.order(&data)?;
    order.apply_assistant_line(heatman.assistant_line_gap());
    let reordered_data = data.reorder(&order);
    let items = mapper(&reordered_data);
    for item in items {
        println!("{item}");
    }
    Ok(())
}

fn rs_main(args: Vec<String>) -> Result<()> {
    let heatman: Heatman = Parser::try_parse_from(args)
        .map_err(|e| Error::Clap(e))
        .and_then(|h: Heatman| h.validate())?;
    match heatman.mode() {
        cli::Mode::Scaler => generate_scaler(heatman),
        cli::Mode::Heatmap => generate_heatmap(heatman),
        cli::Mode::Rows => generate_items(heatman, |data| data.row_headers()),
        cli::Mode::Columns => generate_items(heatman, |data| data.col_headers()),
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