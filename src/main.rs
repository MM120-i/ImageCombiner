mod args;

use args::Args;
use image::{DynamicImage, GenericImageView, ImageFormat, ImageReader, imageops::FilterType::Triangle, ImageError};
use std::convert::TryInto;


#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
    UnableToReadImageFromPath(std::io::Error),
    UnableToFormatImage(String),
    UnableToDecodeImage(ImageError),
    UnableToSaveImage(ImageError),
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity: u32 = width * height * 4;
        let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity.try_into().unwrap());

        FloatingImage {
            width,
            height,
            data: buffer,
            name,
        }
    }

    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }

        self.data = data;
        Ok(())
    }
}

fn main(){
    if let Err(e) = run() {
        print_error(e);
    }
}

fn run() -> Result<(), ImageDataErrors> {
    let args: Args = Args::new();
    let (image_1, image_format1) = find_image_from_path(args.image_1)?;
    let (image_2, image_format2) = find_image_from_path(args.image_2)?;

    if image_format1 != image_format2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image_1, image_2) = standardize_size(image_1, image_2);
    let mut output: FloatingImage = FloatingImage::new(image_1.width(), image_1.height(), args.output);
    let combined_data: Vec<u8> = combine_images(image_1, image_2);

    output.set_data(combined_data)?;

    if let Err(e) = image::save_buffer_with_format(output.name, &output.data, output.width, output.height, image::ColorType::Rgba8, image_format1) {
        Err(ImageDataErrors::UnableToSaveImage(e))
    }
    else{
        Ok(())
    }
}

fn print_error(err: ImageDataErrors){
    match err {
        ImageDataErrors::DifferentImageFormats => {
            eprintln!("Error: Images must be the same format");
        }
        ImageDataErrors::BufferTooSmall => {
            eprintln!("Error: Image buffer is too small");
        }
        ImageDataErrors::UnableToDecodeImage(io_err) => {
            eprintln!("Error: Cannot decode the image: {}", io_err);
        }
        ImageDataErrors::UnableToFormatImage(path) => {
            eprintln!("Error: Unable to format the image for: {}", path);
        }
        ImageDataErrors::UnableToReadImageFromPath(io_err) => {
            eprintln!("Error: Reading image: {}", io_err);
        }
        ImageDataErrors::UnableToSaveImage(img_err) => {
            eprintln!("Error: Unable to save image: {}", img_err);
        }
    }
}

fn find_image_from_path(path: String) -> Result<(DynamicImage, ImageFormat), ImageDataErrors>{
    match ImageReader::open(&path) {
        Ok(image_reader) => {
            if let Some(image_format) = image_reader.format() {
                match image_reader.decode() {
                    Ok(image) => Ok((image, image_format)),
                    Err(e) => Err(ImageDataErrors::UnableToDecodeImage(e))
                }
            }
            else{
                return Err(ImageDataErrors::UnableToFormatImage(path))
            }
        },
        Err(e) => {
            Err(ImageDataErrors::UnableToReadImageFromPath(e))
        }
    }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1: u32 = dim_1.0 * dim_1.1;
    let pix_2: u32 = dim_2.0 * dim_2.1;

    return if pix_1 < pix_2 {dim_1} else {dim_2};
}

fn standardize_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage){
    let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
    println!("Width: {}, height: {}\n", width, height);

    if image_2.dimensions() == (width, height) {
        (image_1.resize_exact(width, height, Triangle), image_2)
    }
    else{
        (image_1, image_2.resize(width, height, Triangle))
    }
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1: Vec<u8> = image_1.to_rgba8().into_vec();
    let vec_2: Vec<u8> = image_2.to_rgba8().into_vec();

    alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut combined_data: Vec<u8> = vec![0u8; vec_1.len()];
    let mut i: usize = 0;

    while i < vec_1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..= i + 3, set_rgba(&vec_1, i, i + 3));
        }
        else if i + 3 < vec_2.len() {
            combined_data.splice(i..= i + 3, set_rgba(&vec_2, i, i + 3));
        }

        i += 4;
    }

    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba: Vec<u8>= Vec::new();

    for i in start..= end {
        let value: u8 = match vec.get(i){
            Some(d) => *d,
            None => panic!("Index out of bounds")
        };

        rgba.push(value);
    }

    rgba
}