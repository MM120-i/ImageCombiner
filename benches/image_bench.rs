use criterion::{criterion_group, criterion_main, Criterion};
use image_combiner::{find_image_from_path, standardize_size, combine_images, FloatingImage};

// explain syntax
fn bench_load_image_100(c: &mut Criterion){
    c.bench_function("load_image_100x100", |b: &mut criterion::Bencher<'_>| {
        b.iter(|| find_image_from_path(std::hint::black_box("images/image1.png".to_string())))
    });
}

fn bench_load_image_1k(c: &mut Criterion){
    c.bench_function("load_image_100x100", |b: &mut criterion::Bencher<'_>| {
        b.iter(|| find_image_from_path(std::hint::black_box("image/image1.png".to_string())))
    });
}

fn bench_load_image_4k(c: &mut Criterion){
    c.bench_function("load_image_3840x2160", |b: &mut criterion::Bencher<'_>| {
        b.iter(|| find_image_from_path(std::hint::black_box("images/image1.png".to_string())))
    });
}

fn bench_combine_small(c: &mut Criterion){
    let (img_1, _) = find_image_from_path("images/image1.png".to_string()).expect("Failed to load image");
    let (img_2, _) = find_image_from_path("images/image2.png".to_string()).expect("Failed to load image");

    c.bench_function("combine_100x100", |b: &mut criterion::Bencher<'_>| {
        b.iter(|| combine_images(std::hint::black_box(img_1.clone()), std::hint::black_box(img_2.clone())))
    });
}

fn bench_combine_medium(c: &mut Criterion){
    let (img_1, _) = find_image_from_path("images/image1.png".to_string()).expect("Failed to load image");
    let (img_2, _) = find_image_from_path("images/image2.png".to_string()).expect("Failed to load image");

    c.bench_function("combine_1000x1000", |b: &mut criterion::Bencher<'_>| {
        b.iter(|| combine_images(std::hint::black_box(img_1.clone()), std::hint::black_box(img_2.clone())))
    });
}

fn bench_combine_large(c: &mut Criterion){
    let (img_1, _) = find_image_from_path("images/image1.png".to_string()).expect("Failed to load image");
    let (img_2, _) = find_image_from_path("images/image2.png".to_string()).expect("Failed to load image");

    c.bench_function("combine_3840x2160", |b: &mut criterion::Bencher<'_>| {
        b.iter(|| combine_images(std::hint::black_box(img_1.clone()), std::hint::black_box(img_2.clone())))
    });
}

fn bench_full_pipeline(c: &mut Criterion){
    c.bench_function("full_pipeline_100x100", |b: &mut criterion::Bencher<'_>| {
        b.iter(|| {
            let (img_1, _fmt1) = find_image_from_path("image/image1.png".to_string()).expect("Failed to load image");
            let (img_2, _fmt1) = find_image_from_path("image/image2.png".to_string()).expect("Failed to load image");

            let (img_1, img_2) = standardize_size(img_1, img_2);
            combine_images(img_1, img_2)
        })
    });
}

fn bench_buffer_alloc(c: &mut Criterion){
    c.bench_function("buffer_alloc_100x100", |b: &mut criterion::Bencher<'_>| {
        b.iter(|| FloatingImage::new(100, 100, "test.png".to_string()))
    });

    c.bench_function("buffer_alloc_1000x1000", |b: &mut criterion::Bencher<'_>| {
        b.iter(|| FloatingImage::new(1000, 1000, "test.png".to_string()))
    });

    c.bench_function("buffer_alloc_3840x2160", |b: &mut criterion::Bencher<'_>| {
        b.iter(|| FloatingImage::new(3840, 2160, "test.png".to_string()))
    });
}

criterion_group!(
    benches,
    bench_load_image_100,
    bench_load_image_1k,
    bench_load_image_4k,
    bench_combine_small,
    bench_combine_medium,
    bench_combine_large,
    bench_full_pipeline,
    bench_buffer_alloc
);

criterion_main!(benches);