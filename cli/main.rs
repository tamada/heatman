use crate::cli::Heatman;
use clap::Parser;

use heatman::{Error, Data, Result};
use image::{ImageBuffer, Rgba};

mod cli;

fn output_image(data: Data<Rgba<u8>>, heatman: Heatman) -> Result<()> {
    let pixel = heatman.pixel() as usize;
    let img_width = data.image_width(pixel) as u32;
    let img_height = data.image_height(pixel) as u32;
    log::info!("Output image size: {}x{}", img_width, img_height);
    let mut py = 0;
    let gap_color = Rgba([0, 0, 0, 255]);
    let mut result_image = ImageBuffer::new(img_width, img_height);
    for (y, row_is_gap) in data.gapps_row(pixel).into_iter().enumerate() {
        if row_is_gap {
            for px in 0..img_width {
                result_image.put_pixel(px, py, gap_color);
            }
            py += 1;
        }
        let mut px = 0;
        for (x, col_is_gap) in data.gapps_col(pixel).into_iter().enumerate() {
            if col_is_gap {
                result_image.put_pixel(px, py, gap_color);
                px += 1;
            }
            let pixel = data.cell(y, x)
                .map(|c| *c)
                .unwrap_or(gap_color);
            result_image.put_pixel(px, py, pixel);
            px += 1;
        }
        py += 1;
    }

    result_image.save(heatman.dest())
        .map_err(|e| Error::Image(e))
}

fn generate_scaler(heatman: Heatman) -> Result<()> {
    let range = 0_i32..240_i32;
    let line = range.into_iter()
        .map(|i| Some(i as f64 / 240.0))
        .collect::<Vec<Option<f64>>>();
    let table = (0..10).into_iter()
        .map(|_| line.clone())
        .collect::<Vec<Vec<Option<f64>>>>();
    let data = Data::new(table);
    let rgbdata: Data<Rgba<u8>> = data.into();
    output_image(rgbdata, heatman)
}

fn generate_heatmap(heatman: Heatman) -> Result<()> {
    let data = heatman.load_image()?;
    let rgbdata: Data<Rgba<u8>> = data.into();
    output_image(rgbdata, heatman)
}

fn rs_main(args: Vec<String>) -> Result<()> {
    let heatman: Heatman = Parser::try_parse_from(args)
        .map_err(|e| Error::Clap(e))
        .and_then(|h: Heatman| h.validate())?;
    if heatman.is_scaler_mode() {
        generate_scaler(heatman)
    } else {
        generate_heatmap(heatman)
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