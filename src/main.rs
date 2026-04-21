mod args;

use args::Args;
use log::info;

use image_combiner::{
    find_image_from_path,
    standardize_size,
    combine_images,
    FloatingImage,
    ImageDataErrors,
};

fn init_logger() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
}

fn main() {
    init_logger();

    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    let (image_1, image_format1) = find_image_from_path(args.image_1)?;
    let (image_2, image_format2) = find_image_from_path(args.image_2)?;

    if image_format1 != image_format2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image_1, image_2) = standardize_size(image_1, image_2);
    let width: u32 = image_1.width();
    let height: u32 = image_1.height();
    let mut output: FloatingImage = FloatingImage::new(width, height, args.output);
    let combined_data: Vec<u8> = combine_images(image_1, image_2);

    output.set_data(combined_data)?;

    if let Err(e) = image::save_buffer_with_format(output.name(), output.data(), output.width(), output.height(),image::ColorType::Rgba8,image_format1) {
        Err(ImageDataErrors::UnableToSaveImage(e))
    } else {
        info!(
            "Saved combined image: {}x{} -> {}",
            output.width(),
            output.height(),
            output.name()
        );
        
        Ok(())
    }
}