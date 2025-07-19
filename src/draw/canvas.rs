use crate::draw::Color;
use std::{convert, fmt};

#[derive(Debug, Clone, Copy)]
struct ColorU8 {
    red: u8,
    green: u8,
    blue: u8,
}

impl ColorU8 {
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
            red: (value.red.clamp(0.0, 1.0) * 255.0) as u8,
            green: (value.green.clamp(0.0, 1.0) * 255.0) as u8,
            blue: (value.blue.clamp(0.0, 1.0) * 255.0) as u8,
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
    pub fn color_at(&self, x: usize, y: usize) -> Option<&Color> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&self.pixels[y * self.width + x])
    }

    /// Writes a pixel with a given Color at a given location (0-indexed)
    pub fn write_pixel_at(&mut self, x: usize, y: usize, color: &Color) -> Result<(), CanvasError> {
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
        let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);

        for y in 0..self.height {
            let line = (0..self.width)
                .map(|x| {
                    let color = self.color_at(x, y).unwrap();
                    format!("{}", ColorU8::from(color))
                })
                .collect::<Vec<_>>()
                .join(" ");

            let line_len = 

            ppm.push_str(&line);
            ppm.push('\n');
        }

        ppm
    }
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
            .write_pixel_at(2, 3, &color_red)
            .expect("Fail to assign color to canvas pixel");

        let pixel = canvas
            .color_at(2, 3)
            .expect("Fail to get pixel from canvas");

        assert_eq!(*pixel, color_red);
    }

    #[test]
    fn constructing_ppm_header() {
        let canvas = Canvas::new(5, 3);
        let ppm = canvas.to_ppm();
    }
}
