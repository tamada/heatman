use heatman::{Heatmap, Data, DataLoader};
use image::{ImageBuffer, Rgba};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = DataLoader::load("testdata/sample.csv")
        .expect("Failed to load data from CSV");
    let cells: Data<Rgba<u8>> = data.into();
    let context = Heatmap::new(cells, 10);
    let image: ImageBuffer<Rgba<u8>, Vec<u8>> = context.into();
    image.save("output.png")
        .expect("Failed to save heatmap image");
    Ok(())
}
