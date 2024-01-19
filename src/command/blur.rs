use termcolor::{Color, ColorSpec, StandardStream, WriteColor};
use std::io::Write;
use crate::util;

pub fn blur_cmd_handler(stdout: &mut StandardStream, file_path: String, strength: Option<f32>) -> Result<(), util::error::FreneError> {
    stdout.set_color(&mut ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;
    write!(stdout, "   Reading & Analyzing ")?;
    stdout.set_color(&mut ColorSpec::new())?;
    writeln!(stdout, "{}", file_path)?;

    let start = std::time::Instant::now();
    let img = image::open(file_path)?;
    let power = strength.unwrap_or(5.0);
    let output = String::from("./blur-result.png");

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

    util::gaussian_blur(&mut data_new, width as usize, height as usize, power);
    image::save_buffer(&output, &data_new.into_iter().flatten().collect::<Vec<u8>>(), width as u32, height as u32, image::ColorType::Rgb8)?;

    let duration = start.elapsed();

    stdout.set_color(&mut ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;
    write!(stdout, "    Finished ")?;
    stdout.set_color(&mut ColorSpec::new())?;
    writeln!(stdout, "created file \"{}\" in {:.2}s", output, duration.as_secs_f32())?;
    Ok(())
}