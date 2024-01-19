use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use crate::util;

pub fn blur_cmd_handler(stdout: &mut StandardStream, file_path: String, strength: f32) -> Result<(), image::ImageError> {

    let img = image::open(file_path)?;

    let width = img.width() as usize;
    let height = img.height() as usize;
    let rgb_img = img.into_rgb8();
    let data = rgb_img.as_raw();
    let mut data_new = Vec::<[u8; 3]>::with_capacity(width * height);
    data_new.resize(width * height, [0, 0, 0]);

    for y in 0..height {
        for x in 0..width {
            let offset = ((width * y) + x) as usize;
            data_new[((width * y) + x) as usize] = [
                data[offset * 3],
                data[(offset * 3) + 1],
                data[(offset * 3) + 2],
            ];
        }
    }

    util::gaussian_blur(&mut data_new, width as usize, height as usize, strength);
    image::save_buffer("./test.png", data_new, width, height, image::ColorType::Rgb8)?;

    Ok(())
}