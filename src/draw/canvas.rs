use crate::draw::Color;
use std::{convert, fmt};

#[derive(Debug, Clone, Copy)]
struct ColorU8 {
    red: u8,
    green: u8,
    blue: u8,
}

impl ColorU8 {
    #[allow(dead_code)]
    fn new(red: u8, green: u8, blue: u8) -> ColorU8 {
        ColorU8 { red, green, blue }
    }
}

impl fmt::Display for ColorU8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.red, self.green, self.blue)
    }
}

impl convert::From<&Color> for ColorU8 {
    fn from(value: &Color) -> Self {
        ColorU8 {
            red: (value.red.clamp(0.0, 1.0) * 255.0).round() as u8,
            green: (value.green.clamp(0.0, 1.0) * 255.0).round() as u8,
            blue: (value.blue.clamp(0.0, 1.0) * 255.0).round() as u8,
        }
    }
}

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>, // grid of colors, row major
}

#[derive(Debug)]
pub enum CanvasError {
    OutOfBounds {
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    },
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }

    /// Returns the color of a pixel at a given location (0-indexed)
    pub fn get_color_at(&self, x: usize, y: usize) -> Option<&Color> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&self.pixels[y * self.width + x])
    }

    /// Writes a pixel with a given Color at a given location (0-indexed)
    pub fn set_pixel_at(&mut self, x: usize, y: usize, color: &Color) -> Result<(), CanvasError> {
        if x >= self.width || y >= self.height {
            return Err(CanvasError::OutOfBounds {
                x,
                y,
                width: self.width,
                height: self.height,
            });
        }

        self.pixels[y * self.width + x] = color.to_owned();

        Ok(())
    }

    /// Write the content of canvas to a PPM P3-formatted string
    pub fn to_ppm(&self) -> String {
        const MAX_LINE_LEN: usize = 70;
        let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);

        for y in 0..self.height {
            let line = (0..self.width)
                .map(|x| {
                    let color = self.get_color_at(x, y).unwrap();
                    format!("{}", ColorU8::from(color))
                })
                .collect::<Vec<_>>()
                .join(" ");

            let line = wrap_string(&line, MAX_LINE_LEN);

            ppm.push_str(&line);
            ppm.push('\n');
        }
        dbg!(&ppm);
        ppm
    }
}

fn wrap_string(str: &str, max_len: usize) -> String {
    let mut result = String::new();
    let mut line_start: usize = 0;

    while line_start <= str.len() {
        let mut line_end = line_start + max_len;

        if line_end >= str.len() {
            result.push_str(str.get(line_start..str.len()).unwrap_or_default());
            break;
        }

        while !str.is_char_boundary(line_end) || str.get(line_end..line_end + 1) != Some(" ") {
            line_end -= 1;

            if line_end == line_start {
                break;
            }
        }
        if line_end == line_start {
            break;
        }

        result.push_str(str.get(line_start..line_end).unwrap_or_default());
        result.push('\n');

        line_start = line_end + 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let canvas = Canvas::new(10, 20);

        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);

        for c in canvas.pixels {
            assert_eq!(c, Color::new(0.0, 0.0, 0.0));
        }
    }

    #[test]
    fn write_pixels_to_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let color_red = Color::new(1.0, 0.0, 0.0);

        canvas
            .set_pixel_at(2, 3, &color_red)
            .expect("Should be able to assign color to canvas pixel");

        let pixel = canvas
            .get_color_at(2, 3)
            .expect("Should be able to get pixel from canvas");

        assert_eq!(*pixel, color_red);
    }

    #[test]
    fn wrap_a_text() {
        let input_str = String::from(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153",
        );
        let line_length: usize = 70;
        let expected_str = String::from(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153",
        );

        assert_eq!(wrap_string(&input_str, line_length), expected_str);
    }

    #[test]
    fn constructing_ppm_header() {
        let canvas = Canvas::new(5, 3);
        let ppm = canvas.to_ppm();

        let mut lines = ppm.lines();
        assert_eq!(lines.next(), Some("P3"));
        assert_eq!(lines.next(), Some("5 3"));
        assert_eq!(lines.next(), Some("255"));
    }

    #[test]
    fn constructing_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        canvas
            .set_pixel_at(0, 0, &c1)
            .expect("Should be able to set color to canvas");
        canvas
            .set_pixel_at(2, 1, &c2)
            .expect("Should be able to set color to canvas");
        canvas
            .set_pixel_at(4, 2, &c3)
            .expect("Should be able to set color to canvas");

        let ppm = canvas.to_ppm();
        let mut lines = ppm.lines();

        assert_eq!(lines.nth(3), Some("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"));
        assert_eq!(lines.next(), Some("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0"));
        assert_eq!(lines.next(), Some("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"));
    }

    #[test]
    fn splitting_long_lines_in_ppm_data() {
        let canvas_width = 10;
        let canvas_height = 2;
        let mut canvas = Canvas::new(canvas_width, canvas_height);
        let color = Color::new(1.0, 0.8, 0.6);

        for i in 0..canvas_height {
            for j in 0..canvas_width {
                canvas
                    .set_pixel_at(j, i, &color)
                    .expect(&format!("Should be able to set color at ({},{})", j, i));
            }
        }

        let ppm = canvas.to_ppm();
        let mut lines = ppm.lines();

        assert_eq!(lines.nth(3), Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"));
        assert_eq!(lines.next(), Some("153 255 204 153 255 204 153 255 204 153 255 204 153"));
        assert_eq!(lines.next(), Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"));
        assert_eq!(lines.next(), Some("153 255 204 153 255 204 153 255 204 153 255 204 153"));
    }

    #[test]
    fn ppm_ends_with_newline() {
        let canvas_width = 10;
        let canvas_height = 2;
        let mut canvas = Canvas::new(canvas_width, canvas_height);
        let color = Color::new(1.0, 0.8, 0.6);

        for i in 0..canvas_height {
            for j in 0..canvas_width {
                canvas
                    .set_pixel_at(j, i, &color)
                    .expect(&format!("Should be able to set color at ({},{})", j, i));
            }
        }

        let ppm = canvas.to_ppm();
        let last_char = ppm.get(ppm.len()-1..);

        assert_eq!(last_char, Some("\n"));
    }
}
