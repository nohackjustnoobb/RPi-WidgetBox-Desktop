use std::io::Cursor;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use image::{DynamicImage, ImageFormat};

pub fn image_as_base64(img: &DynamicImage) -> Option<String> {
    let mut buf = Cursor::new(Vec::new());

    let result = img.write_to(&mut buf, ImageFormat::WebP);
    result.ok()?;

    Some(STANDARD.encode(buf.get_ref()))
}
