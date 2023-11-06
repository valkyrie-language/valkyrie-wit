use super::*;
use crate::exports::vit::image::rgba_image::{OwnRgbaImage, RgbaPixel};
use image::Rgba;
use wit_bindgen::Resource;

impl From<Rgba<u8>> for RgbaPixel {
    fn from(value: Rgba<u8>) -> Self {
        RgbaPixel { r: value[0], g: value[1], b: value[2], a: value[3] }
    }
}

impl From<RgbaPixel> for Rgba<u8> {
    fn from(value: RgbaPixel) -> Self {
        Rgba([value.r, value.g, value.b, value.a])
    }
}

impl GuestRgbaImage for RgbaImageHost {
    fn new(init: Vec<u32>) -> Self {
        RgbaImageHost { ptr: RgbaImage::new(0, 0) }
    }

    fn new_w_h(&self, w: u32, h: u32) -> OwnRgbaImage {
        Resource::new(RgbaImageHost { ptr: RgbaImage::new(w, h) })
    }

    fn decode_png(bytes: Vec<u8>) -> OwnRgbaImage {
        todo!()
    }

    fn get_pixel(&self, x: u32, y: u32) -> RgbaPixel {
        self.ptr.get_pixel(x, y).clone().into()
    }

    fn set_pixel(&self, x: u32, y: u32, p: RgbaPixel) {
        todo!()
        // self.ptr.put_pixel(x, y, p.into());
    }
}
