use std::path::Path;

use crate::Image;

pub fn load_image(input_path: &Path) -> Image {
    image::open(input_path)
        .expect(&format!("Load the image from {:?}", input_path))
        .into_rgba8()
}

pub fn save_image(img: Image, output_path: &Path) {
    img.save(output_path)
        .expect("Save the image to {output_path}");
}
