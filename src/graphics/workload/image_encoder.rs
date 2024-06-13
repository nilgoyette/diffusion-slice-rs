use image::{ImageBuffer, RgbaImage};

use super::*;

impl Context {
    pub(super) fn save_image<'a>(&self, bytes: Vec<u8>) {
        let (width, height) = self.client.img_size;

        let img: RgbaImage = ImageBuffer::from_raw(width, height, bytes)
            .expect("Data size doesn't match dimensions");

        img.save(&self.client.output_path)
            .expect("Failed to save the image");
    }
}
