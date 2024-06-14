use super::*;

impl Context {
    pub(super) fn save_image(&self, bytes: Vec<u8>) {
        let (width, height) = self.client.img_size;

        let img =
            Image::from_raw(width, height, bytes).expect("Data size doesn't match dimensions");

        img.save(&self.client.output_path)
            .expect("Failed to save the image");
    }
}
