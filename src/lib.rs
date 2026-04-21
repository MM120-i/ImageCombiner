use image::{DynamicImage, GenericImageView, ImageFormat, ImageReader, imageops::FilterType::Triangle, ImageError};

#[derive(Debug)]
pub enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
    UnableToReadImageFromPath(std::io::Error),
    UnableToFormatImage(String),
    UnableToDecodeImage(ImageError),
    UnableToSaveImage(ImageError),
}

impl std::fmt::Display for ImageDataErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ImageDataErrors::DifferentImageFormats => write!(f, "Images must be the same format"),
            ImageDataErrors::BufferTooSmall => write!(f, "Image buffer is too small"),
            ImageDataErrors::UnableToReadImageFromPath(e) => write!(f, "Cannot read image: {}", e),
            ImageDataErrors::UnableToFormatImage(path) => write!(f, "Unknown format for: {}", path),
            ImageDataErrors::UnableToDecodeImage(e) => write!(f, "Cannot decode image: {}", e),
            ImageDataErrors::UnableToSaveImage(e) => write!(f, "Cannot save image: {}", e),
        }
    }
}

impl std::error::Error for ImageDataErrors {}

pub struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    pub fn new(width: u32, height: u32, name: String) -> Self {
        let capacity: usize = (width as u64 * height as u64 * 4) as usize;
        let data: Vec<u8> = Vec::with_capacity(capacity);

        FloatingImage {
            width,
            height,
            data,
            name,
        }
    }

    pub fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }

        self.data = data;
        Ok(())
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

pub fn find_image_from_path(path: String) -> Result<(DynamicImage, ImageFormat), ImageDataErrors> {
    match ImageReader::open(&path) {
        Ok(image_reader) => {
            if let Some(image_format) = image_reader.format() {
                match image_reader.decode() {
                    Ok(image) => Ok((image, image_format)),
                    Err(e) => Err(ImageDataErrors::UnableToDecodeImage(e)),
                }
            } 
            else {
                Err(ImageDataErrors::UnableToFormatImage(path))
            }
        }

        Err(e) => Err(ImageDataErrors::UnableToReadImageFromPath(e)),
    }
}

pub fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1: u32 = dim_1 .0 * dim_1.1;
    let pix_2: u32 = dim_2 .0 * dim_2.1;

    if pix_1 < pix_2 {
        dim_1
    } 
    else {
        dim_2
    }
}

pub fn standardize_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());

    if image_2.dimensions() == (width, height) {
        (image_1.resize_exact(width, height, Triangle), image_2)
    } 
    else {
        (image_1, image_2.resize(width, height, Triangle))
    }
}

pub fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1: Vec<u8> = image_1.to_rgba8().into_vec();
    let vec_2: Vec<u8> = image_2.to_rgba8().into_vec();

    alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut combined_data: Vec<u8> = vec![0u8; vec_1.len()];
    let mut i: usize = 0;

    while i < vec_1.len() {
        if i.is_multiple_of(8) {
            combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
        } 
        else if i + 3 < vec_2.len() {
            combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
        }

        i += 4;
    }

    combined_data
}

fn set_rgba(vec: &[u8], start: usize, end: usize) -> Vec<u8> {
    let mut rgba: Vec<u8> = Vec::new();

    for i in start..=end {
        let value = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index out of bounds"),
        };

        rgba.push(value);
    }

    rgba
}