use std::ops::{Add, Mul, Sub};

use image::{ImageBuffer, Rgb, RgbImage};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(pub f64, pub f64, pub f64);

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl From<Color> for Rgb<u8> {
    fn from(value: Color) -> Self {
        let r = ((value.0 * 255.).round() as u8).clamp(0, 255);
        let g = ((value.1 * 255.).round() as u8).clamp(0, 255);
        let b = ((value.2 * 255.).round() as u8).clamp(0, 255);
        Rgb([r, g, b])
    }
}

impl From<Rgb<u8>> for Color {
    fn from(value: Rgb<u8>) -> Self {
        let [r, g, b] = value.0;
        let r = r as f64 / 255.;
        let g = g as f64 / 255.;
        let b = b as f64 / 255.;
        Color(r, g, b)
    }
}

pub struct Canvas {
    width: u32,
    height: u32,
    pixels: RgbImage,
}

impl Canvas {
    pub fn new(width: u32, height: u32, color: Option<Color>) -> Self {
        let mut pixels = ImageBuffer::new(width, height);
        match color {
            None => {}
            Some(color) => {
                for i in 0..height {
                    for j in 0..width {
                        pixels.put_pixel(j, i, Rgb::from(color));
                    }
                }
            }
        }
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.pixels.put_pixel(y, x, Rgb::from(color));
    }

    fn pixel_at(&self, x: u32, y: u32) -> Color {
        Color::from(self.pixels[(y, x)])
    }

    pub fn save(&self, path: &str) -> image::ImageResult<()> {
        self.pixels.save(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn eq(c1: Color, c2: Color) -> bool {
        (c1.0 - c2.0).abs() < 1e6 && (c1.1 - c2.1).abs() < 1e6 && (c1.2 - c2.2).abs() < 1e6
    }
    #[test]
    fn op_colors() {
        let a = Color(0.9, 0.6, 0.75);
        let b = Color(0.7, 0.1, 0.25);
        assert!(eq(a * 2., Color(1.8, 1.2, 1.5)));
        assert!(eq(a + b, Color(1.6, 0.7, 1.)));
        assert!(eq(a - b, Color(0.2, 0.5, 0.5)));
    }

    #[test]
    fn create_canvas() {
        let c = Canvas::new(10, 20, None);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for i in 0..c.height {
            for j in 0..c.width {
                assert_eq!(c.pixel_at(i, j), Color(0., 0., 0.))
            }
        }
    }

    #[test]
    fn create_canvas_red() {
        let c = Canvas::new(10, 20, Some(Color(1., 0., 0.)));
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for i in 0..c.height {
            for j in 0..c.width {
                assert_eq!(c.pixel_at(i, j), Color(1., 0., 0.))
            }
        }
    }
    #[test]
    fn write_canvas() {
        let mut c = Canvas::new(10, 20, None);
        let red = Color(1., 0., 0.);
        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn save_canvas() {
        let c = Canvas::new(100, 100, Some(Color(1., 0., 0.)));
        c.save("img.png").unwrap();
    }
}
