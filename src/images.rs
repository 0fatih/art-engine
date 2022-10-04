use image::{DynamicImage, imageops};

pub fn merge(mut base: DynamicImage, imgs: &[DynamicImage]) -> DynamicImage {
    for img in imgs {
        imageops::overlay(&mut base, img, 0, 0);
    }
    base
}
