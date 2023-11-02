use crate::exports::vit::image::rgba_image::GuestRgbaImage;
use image::RgbaImage;

mod decimal;
pub struct RgbaImageHost {
    ptr: RgbaImage,
}
