use heatman::{Heatmap, ScalerBuilder};
use image::{ImageBuffer, Rgba};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = ScalerBuilder::build_with(20);
    let context = Heatmap::new(data, 1);
    let image: ImageBuffer<Rgba<u8>, Vec<u8>> = context.into();
    image.save("scaler.png")
        .expect("Failed to save scaler image");
    Ok(())
}
