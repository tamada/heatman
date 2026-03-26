use heatman::{Heatmap, Data, DataLoader, Order};
use image::{ImageBuffer, Rgba};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = DataLoader::load("testdata/jaccard.csv")
        .expect("Failed to load data from CSV");
    let order = Order::load_symmetric("testdata/header.csv")
        .expect("Failed to load order from file");
    let reordered_data = data.reorder(&order);
    let cells: Data<Rgba<u8>> = reordered_data.into();
    let context = Heatmap::new(cells, 10);
    let image: ImageBuffer<Rgba<u8>, Vec<u8>> = context.into();
    image.save("jaccard.png")
        .expect("Failed to save heatmap image");
    Ok(())
}
