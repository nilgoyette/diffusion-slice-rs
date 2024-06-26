use std::path::Path;

use crate::Image;

pub fn load_image(input_path: &Path) -> Image {
    match image::open(input_path) {
        Ok(image) => image.into_rgba8(),
        Err(error) => panic!("Image at {input_path:?} could not be loaded: {error}"),
    }
}

pub fn save_image(img: Image, output_path: &Path) {
    if let Err(error) = img.save(output_path) {
        panic!("Failed to save the image at {output_path:?}: {error}");
    }
}
