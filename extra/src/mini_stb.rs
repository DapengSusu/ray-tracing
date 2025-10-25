use std::path::Path;

use image::{GenericImageView, ImageReader};
use palette::{IntoColor, LinSrgb, Srgb};
use thiserror::Error;

#[derive(Debug, Default)]
pub struct LinearPixelImage {
    /// Linear floating point pixel data
    rgb_data: Vec<f32>,
    /// Linear 8-bit pixel data
    rgb_bytes: Vec<u8>,
    /// Loaded image width
    img_width: u32,
    /// Loaded image height
    img_height: u32,
    /// Bytes per scanline
    bytes_per_scanline: u32,
}

/// How many bytes per pixel
const BYTES_PER_PIXEL: u32 = 3;

impl LinearPixelImage {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, MiniStbError> {
        let img = ImageReader::open(path)?.decode()?;

        let (width, height) = img.dimensions();
        let bytes_per_scanline = width * BYTES_PER_PIXEL;
        let rgb8_img = img.to_rgb8();

        let mut linear_data = Self {
            img_width: width,
            img_height: height,
            bytes_per_scanline,
            ..Default::default()
        };

        rgb8_img.pixels().for_each(|pixel| {
            // 将8位整数转换为[0,1]范围的浮点数
            let srgb = Srgb::new(
                pixel[0] as f32 / 255.,
                pixel[1] as f32 / 255.,
                pixel[2] as f32 / 255.,
            );

            // 将sRGB转换到线性RGB空间
            let linear_rgb: LinSrgb = srgb.into_color();

            // 保存浮点数数据
            linear_data.rgb_data.extend_from_slice(&[
                linear_rgb.red,
                linear_rgb.green,
                linear_rgb.blue,
            ]);

            // 生成字节数据
            linear_data.rgb_bytes.extend_from_slice(&[
                (linear_rgb.red * 255.).round() as u8,
                (linear_rgb.green * 255.).round() as u8,
                (linear_rgb.blue * 255.).round() as u8,
            ]);
        });

        Ok(linear_data)
    }

    /// Return the three RGB bytes of the pixel at x,y.
    /// If there is no image data, returns magenta (RGB: (228, 0, 127)).
    pub fn pixel_data_at(&self, x: u32, y: u32) -> PixelRgb8 {
        let index = y * self.bytes_per_scanline + x * BYTES_PER_PIXEL;

        self.rgb_bytes
            .iter()
            .skip(index as usize)
            .take(3)
            .copied()
            .collect::<PixelRgb8>()
    }

    pub fn width(&self) -> u32 {
        self.img_width
    }

    pub fn height(&self) -> u32 {
        self.img_height
    }
}

#[derive(Debug, Error)]
pub enum MiniStbError {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("`image` crate error: {0}")]
    ImageError(#[from] image::ImageError),
}

/// An RGB pixel with 8-bit color channels.
pub struct PixelRgb8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl FromIterator<u8> for PixelRgb8 {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        PixelRgb8 {
            r: iter.next().unwrap_or(255),
            g: iter.next().unwrap_or(255),
            b: iter.next().unwrap_or(255),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_image_should_work() {
        let path = "../assets/earthmap.jpg";
        let pixel_data = LinearPixelImage::load(path).unwrap();

        assert_eq!(pixel_data.img_width, 1024);
        assert_eq!(pixel_data.img_height, 512);
        assert_eq!(pixel_data.bytes_per_scanline, 1024 * BYTES_PER_PIXEL);
    }
}
