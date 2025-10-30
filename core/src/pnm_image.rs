use std::{
    fmt::Display,
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
    sync::atomic::{AtomicU32, Ordering},
    time::Instant,
};

use rayon::prelude::*;

use crate::Renderer;

#[derive(Debug, Clone, Hash)]
pub struct PnmImage {
    header: PnmHeader,
    data: Vec<Rgb>,
}

impl PnmImage {
    pub fn new(magic: PnmFormat, width: u32, height: u32) -> Self {
        Self {
            header: PnmHeader::new(magic, width, height),
            data: Vec::new(),
        }
    }

    pub fn with_capacity(magic: PnmFormat, width: u32, height: u32, capacity: usize) -> Self {
        Self {
            header: PnmHeader::new(magic, width, height),
            data: Vec::with_capacity(capacity),
        }
    }

    /// 创建空的 PPM 图像
    pub fn new_ppm_ascii(width: u32, height: u32) -> Self {
        Self::new(PnmFormat::P3, width, height)
    }

    /// 按照指定的像素处理器生成并保存像素数据
    pub fn generate<P: Renderer>(&mut self, processor: P) {
        // 用于显示处理进度
        let remaining_lines = AtomicU32::new(self.header.height);

        // 开始统计处理用时
        let now = Instant::now();

        let rows = (0..self.header.height)
            .into_par_iter() // rayon parallelize
            .flat_map(|j| {
                let row = (0..self.header.width)
                    .map(|i| processor.render(i, j))
                    .collect::<Vec<_>>();

                let remaining = remaining_lines.fetch_sub(1, Ordering::Relaxed);
                eprint!("\r\x1B[KScanlines remaining: {}", remaining - 1);

                row
            })
            .collect::<Vec<_>>();

        // 保存像素数据
        self.add_pixels(&rows);

        // 结束计时
        eprint!("\r\x1B[K");
        let elapsed = now.elapsed();
        eprintln!("\nDone. Elapsed time: {}ms", elapsed.as_millis());
    }

    /// 为存放像素数据提前分配空间，最终空间大于等于 data.len() + additional，
    /// 如果容量已经足够，则什么都不做。
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` _bytes_.
    fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    fn add_pixels(&mut self, pixels: &[Rgb]) {
        self.reserve(pixels.len());
        self.data.extend(pixels.iter());
    }

    /// 将图像数据写入到标准输出中。
    /// 写入之前必须先调用 `self.generate(...)` 生成数据。
    pub fn write_to_stdout(&self) -> Result<(), io::Error> {
        self.write_to(&mut io::stdout().lock())
    }

    /// 将图像数据写入到指定文件中，文件不存在则创建，存在则截断。
    /// 写入之前必须先调用 `self.generate(...)` 生成数据。
    pub fn write_to_file<P: AsRef<Path>>(&self, filename: P) -> Result<(), io::Error> {
        self.write_to(&mut File::create(filename)?)
    }

    /// 将图像数据写入到指定 buffer 中，如 File，stdout 等。
    /// 写入之前必须先调用 `self.generate(...)` 生成数据。
    ///
    /// # Panics
    ///
    /// 图像像素总数（width*height）和 `data.len()` 不相等会导致 panic
    pub fn write_to<W: Write>(&self, w: &mut W) -> Result<(), io::Error> {
        let pixel_count = self.header.pixel_count();
        if pixel_count as usize != self.data.len() {
            panic!(
                "w: {}, h: {}, expected pixel count: {}, but got: {}",
                self.header.width,
                self.header.height,
                pixel_count,
                self.data.len()
            );
        }

        write!(BufWriter::new(w), "{}", self)
    }

    pub fn image_width(&self) -> u32 {
        self.header.width
    }

    pub fn image_height(&self) -> u32 {
        self.header.height
    }

    pub fn image_format(&self) -> PnmFormat {
        self.header.magic
    }
}

impl Display for PnmImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.header)?;
        self.data
            .iter()
            .try_for_each(|pixel| write!(f, "\n{}", pixel))
    }
}

#[derive(Debug, Clone, Hash)]
struct PnmHeader {
    /// 文件格式及类型
    magic: PnmFormat,
    /// 图像宽度
    width: u32,
    /// 图像高度
    height: u32,
    /// 像素最大颜色，0-255（对应黑到白），PBM 格式没有此字段
    max_color: Option<u8>,
}

impl PnmHeader {
    fn new(magic: PnmFormat, width: u32, height: u32) -> Self {
        let max_color = Self::max_color_by_magic(&magic);
        Self {
            magic,
            width,
            height,
            max_color,
        }
    }

    fn pixel_count(&self) -> u32 {
        self.width * self.height
    }

    fn max_color_by_magic(magic: &PnmFormat) -> Option<u8> {
        match magic {
            PnmFormat::P1 | PnmFormat::P4 => None,
            _ => Some(255),
        }
    }
}

impl From<PnmHeader> for Vec<u8> {
    fn from(header: PnmHeader) -> Self {
        let mut header_bytes = Vec::with_capacity(12);

        header_bytes.extend(header.magic.as_bytes());
        header_bytes.extend(&header.width.to_be_bytes());
        header_bytes.extend(&header.height.to_be_bytes());

        if let Some(max_color) = header.max_color {
            header_bytes.push(max_color);
        }

        header_bytes
    }
}

impl Display for PnmHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.magic {
            PnmFormat::P1 | PnmFormat::P4 => {
                write!(f, "{}\n{} {}", self.magic, self.width, self.height)
            }
            _ => write!(
                f,
                "{}\n{} {}\n{}",
                self.magic,
                self.width,
                self.height,
                self.max_color.unwrap()
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum PnmFormat {
    /// PBM，单色图，ASCII类型
    P1,
    /// PGM，灰度图，ASCII类型
    P2,
    /// PPM，像素图，ASCII类型
    P3,
    /// PBM，单色图，字节码类型
    P4,
    /// PGM，灰度图，字节码类型
    P5,
    /// PPM，像素图，字节码类型
    P6,
}

impl PnmFormat {
    fn as_bytes(&self) -> &[u8] {
        match self {
            PnmFormat::P1 => b"P1",
            PnmFormat::P2 => b"P2",
            PnmFormat::P3 => b"P3",
            PnmFormat::P4 => b"P4",
            PnmFormat::P5 => b"P5",
            PnmFormat::P6 => b"P6",
        }
    }
}

impl Display for PnmFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_pnm_header_size() {
        assert_eq!(size_of::<PnmHeader>(), 12);
    }
}
