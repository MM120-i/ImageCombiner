use image_combiner::{
    get_smallest_dimensions, FloatingImage,
};

#[test]
fn test_get_smallest_dimensions() {
    let dim_1: (u32, u32) = (100, 100);
    let dim_2: (u32, u32) = (200, 200);
    let result: (u32, u32) = get_smallest_dimensions(dim_1, dim_2);
    assert_eq!(result, (100, 100));

    let dim_3: (u32, u32) = (800, 600);
    let dim_4: (u32, u32) = (640, 480);
    let result: (u32, u32) = get_smallest_dimensions(dim_3, dim_4);
    assert_eq!(result, (640, 480));
}

#[test]
fn test_floating_image_new() {
    let img: FloatingImage = FloatingImage::new(100, 100, "test.png".to_string());

    assert_eq!(img.width(), 100);
    assert_eq!(img.height(), 100);
    assert_eq!(img.name(), "test.png");
    assert!(img.data().capacity() >= 100 * 100 * 4);
}

#[test]
fn test_floating_image_set_data() {
    let mut img: FloatingImage = FloatingImage::new(10, 10, "test.png".to_string());
    let data: Vec<u8> = vec![0u8; 10 * 10 * 4];

    let result: Result<(), image_combiner::ImageDataErrors> = img.set_data(data.clone());
    assert!(result.is_ok());
    assert_eq!(img.data(), &data);
}

#[test]
fn test_floating_image_buffer_capacity() {
    let img: FloatingImage = FloatingImage::new(1920, 1080, "test.png".to_string());
    let required: usize = 1920 * 1080 * 4;
    
    assert!(img.data().capacity() >= required);
}