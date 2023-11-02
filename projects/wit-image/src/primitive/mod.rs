use crate::exports::vit::image::rgba_image::{GuestRgbaImage};
use image::RgbaImage;

pub struct RgbaImageHost {
    wrap: RgbaImage,
}

impl GuestRgbaImage for RgbaImageHost {
    fn new(init: Vec<u32>) -> Self {
        RgbaImageHost {
            wrap: RgbaImage::new(0, 0),
        }
    }
}

